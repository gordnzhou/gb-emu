use std::cmp::min;

const TILE_SIZE: usize = 16;
const TILE_ENTRIES: usize = 384;

const OAM_ENTRY_SIZE: usize = 4;
const OAM_ENTRIES: usize = 40;

const TILE_MAP_SIZE: usize = 0x0400;

const LCD_WIDTH: usize= 160;
const LCD_HEIGHT: usize = 144;

const SCAN_LINE_DOTS: u32 = 456;
const MODE_1_DOTS: u32 = SCAN_LINE_DOTS * 10;
const MODE_2_DOTS: u32 = 80;
const MODE_3_MIN_DOTS: u32 = 172;

pub struct Ppu {
    // 16-byte types stored here
    tile_data: [[u8; TILE_SIZE]; TILE_ENTRIES],
    tile_map0: [u8; TILE_MAP_SIZE],
    tile_map1: [u8; TILE_MAP_SIZE],
    oam: [[u8; OAM_ENTRY_SIZE]; OAM_ENTRIES],
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    dma: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,

    // frame buffer representing the LCD screen that will be
    // displayed on canvas at 60 Hz 
    pub frame_buffer: [[u8; LCD_WIDTH]; LCD_HEIGHT],
    pub stat_triggered: bool,
    stat_line: bool,
    mode: u8,
    mode_elapsed_dots: u32,

    mode_3_dots: u32,
    cur_pixel_x: usize,
    wy_cond: bool,
    wx_cond: bool,
    win_counter: usize,

    obj_buffer_index: usize,
    obj_buffer: Vec<usize>,

    pub entered_vblank: bool,
    last_vblank_scanline: u32,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu { 
            tile_data: [[0; TILE_SIZE]; TILE_ENTRIES],
            tile_map0: [0; TILE_MAP_SIZE],
            tile_map1: [0; TILE_MAP_SIZE],
            oam: [[0; OAM_ENTRY_SIZE]; OAM_ENTRIES],
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,

            frame_buffer: [[0; LCD_WIDTH]; LCD_HEIGHT],
            stat_triggered: false,
            stat_line: false,

            mode: 2,
            mode_elapsed_dots: 0,

            mode_3_dots: 0,
            cur_pixel_x: 0,
            wy_cond: false,
            wx_cond: false,
            win_counter: 0,

            obj_buffer_index: 0,
            obj_buffer: Vec::new(),

            entered_vblank: false,
            last_vblank_scanline: 0,
        }
    }

    /// Steps through the PPU over the given period (in cycles).
    pub fn step(&mut self, cycles: u8) {
        self.entered_vblank = false;
        self.stat_triggered = false;

        if self.lcd_ppu_disabled() {
            return;
        }

        let dots = cycles as u32 * 4;
        let next_dots = self.mode_elapsed_dots + dots;

        let mode_end = match self.mode {
            0 => SCAN_LINE_DOTS - self.mode_3_dots - MODE_2_DOTS,
            1 => MODE_1_DOTS,
            2 => MODE_2_DOTS,
            3 => self.mode_3_dots,
            _ => unreachable!()
        };

        if next_dots < mode_end {
            self.step_mode(dots);
            self.mode_elapsed_dots += dots;    
        } else {
            // new mode will be reached immediately after / during current cycle(s)
            self.step_mode(mode_end - self.mode_elapsed_dots);
            self.next_mode();

            // runs if new mode is reached partway through current cycle(s)
            self.step_mode(next_dots - mode_end);
            self.mode_elapsed_dots = next_dots - mode_end;
        }
    }

    // Updates PPU to next natural mode state.
    fn next_mode(&mut self) {
        self.mode = match self.mode {
            0 => {
                self.cur_pixel_x = 0;
                self.obj_buffer = Vec::new();
                self.obj_buffer_index = 0;
                self.ly += 1; 

                if self.ly == LCD_HEIGHT as u8 {
                    // HBlank -> VBlank
                    self.wy_cond = false;
                    self.win_counter = 0;
                    self.entered_vblank = true;
                    self.last_vblank_scanline = 0;

                    1
                } else {
                    // HBlank -> OAM Search
                    self.wy_cond |= self.wy == self.ly;
                    self.win_counter += (self.win_enabled() && self.wy_cond && self.wx_cond) as usize;
                    2
                }
            },
            1 => {
                // VBlank -> OAM Search
                self.ly = 0;
                2 
            },
            2 => {
                // OAM Search -> Drawing
                self.obj_buffer_index = 0;
                self.obj_buffer.sort_by(|a, b| { self.oam[*a][1].cmp(&self.oam[*b][1])});
                self.wx_cond = false;
                self.mode_3_dots = self.calc_mode_3_dots();
                3
            },
            3 => {
                // Drawing -> HBlank
                0
            },
            _ => unreachable!()
        };
    }

    /// ASSUME: self.mode_elapsed_dots + dots will NOT exceed duration of current mode.
    /// Step through period (in dots) over the current mode (do nothing for mode 1 and 0).
    fn step_mode(&mut self, dots: u32) {
        if dots == 0 {
            return;
        }

        if self.mode == 2 {
            // check an attribute from OAM every 2 dots
            let mut fetches = (dots + 1) / 2;

            while fetches > 0 && self.obj_buffer_index < OAM_ENTRIES && self.obj_buffer.len() < 10 {
                let obj_y = self.oam[self.obj_buffer_index][0];
                
                if self.ly + 16 >= obj_y && self.ly + 16 < obj_y + self.obj_size()  {
                    self.obj_buffer.push(self.obj_buffer_index);
                }

                self.obj_buffer_index += 1;
                fetches -= 1;
            }

        } else if self.mode == 3 {
            let mut pixels_left = dots;

            // TODO: implement BG and OAM FIFO 
            while self.cur_pixel_x < LCD_WIDTH && pixels_left > 0 {
                self.wx_cond |= self.wx as usize == self.cur_pixel_x + 7;

                let colour = self.render_pixel(self.cur_pixel_x, self.ly as usize);
                self.frame_buffer[self.ly as usize][self.cur_pixel_x] = colour;

                self.cur_pixel_x += 1;
                pixels_left -= 1;
            }

        }  else if self.mode == 1 {
            if self.last_vblank_scanline + dots >= SCAN_LINE_DOTS {
                self.ly += 1;
            }
            self.last_vblank_scanline = (self.last_vblank_scanline + dots) % SCAN_LINE_DOTS;

        }

        self.update_stat();
    }

    fn render_pixel(&self, lcd_x: usize, lcd_y: usize) -> u8 {
        let mut colour = self.render_bgwin_pixel(lcd_x, lcd_y);
        colour = self.render_obj_pixel(colour, lcd_x, lcd_y);
        colour
    }

    fn render_bgwin_pixel(&self, lcd_x: usize, lcd_y: usize) -> u8 {
        if self.obj_only() {
            return 0;
        }

        let tile_data =  if self.win_enabled() && self.wy_cond && self.wx_cond {
            let x = lcd_x + 7 - self.wx as usize;
            let y = self.win_counter - 1;
            self.fetch_bgwin_tile(x, y, false)
        } else {
            let x = (lcd_x + self.scx as usize) % 0xFF;
            let y = (lcd_y as usize + self.scy as usize) % 0xFF;
            self.fetch_bgwin_tile(x, y, true)
        };

        Ppu::apply_palette(&tile_data, &self.bgp)
    }

    fn render_obj_pixel(&self, bg_colour: u8, lcd_x: usize, lcd_y: usize) -> u8 {
        let mut colour = bg_colour;

        for index in &self.obj_buffer {
            if !self.obj_enabled() {
                break;
            }

            let obj_y = self.oam[*index][0] as usize; // actual screen y pos = obj_y - 16
            let obj_x = self.oam[*index][1] as usize; // actual screen x pos = obj_x - 8
            let tile_id = self.oam[*index][2] as usize;
            let attributes = self.oam[*index][3];
            let x_flip = attributes & 0x20 != 0;
            let y_flip = attributes & 0x40 != 0;

            if !(obj_x <= lcd_x + 8 && lcd_x < obj_x) {
                continue;
            }

            let tile_id = if self.obj_size() == 16 {
                if (lcd_y as usize + 8 >= obj_y) ^ y_flip {
                    tile_id | 0x01
                } else {
                    tile_id & 0xFE
                }
            } else {
                tile_id
            };

            let tile_x = lcd_x + 8 - obj_x;
            let tile_y = lcd_y as usize + 16 - obj_y;
            let id = self.fetch_tile_pixel(false, tile_id as u8, tile_x, tile_y, x_flip, y_flip);

            if id != 0 { 
                let palette = if attributes & 0x10 == 0 { self.obp0 } else { self.obp1 };

                if attributes & 0x80 == 0 {
                    colour = Ppu::apply_palette(&id, &palette);
                } else if colour == 0 {
                    colour = Ppu::apply_palette(&id, &palette);
                }

                break;
            }
        }

        colour
    }

    /// Returns colour id from tile_id's tile at (tile_x, tile_y).
    /// Set addr_mode to false for 0x8000 tile_data addressing, true for 0x8800 addressing.
    fn fetch_tile_pixel(&self, addr_mode: bool, tile_id: u8, tile_x: usize, tile_y: usize, x_flip: bool, y_flip: bool) -> u8 {
        let tile = self.tile_data[if !addr_mode { 
            tile_id as usize
        } else {
            (256 + (tile_id as i8) as i16) as usize
        }];

        let y = if y_flip { (7 - (tile_y as usize & 7)) << 1 } else { (tile_y as usize & 7) << 1 };
        let x =  if x_flip { 1 << (tile_x & 7) } else { 0x80 >> (tile_x % 8) };

        let pixel_lo = (tile[y] & x != 0) as u8;
        let pixel_hi = (tile[y + 1] & x != 0) as u8;

        (pixel_hi << 1) | pixel_lo
    }

    /// Returns colour id of pixel at (x, y) from bg/window tile map.
    fn fetch_bgwin_tile(&self, x: usize, y: usize, is_bg: bool) -> u8 {
        let tmap_addr = (x >> 3) + ((y >> 3) << 5);
        let lcd_bit = if is_bg { 0x08 } else { 0x40 };
        let tile_id = if self.lcdc & lcd_bit == 0 { 
            self.tile_map0[tmap_addr] 
        } else { 
            self.tile_map1[tmap_addr] 
        };

        self.fetch_tile_pixel(self.lcdc & 0x10 == 0, tile_id, x, y, false, false)
    }

    /// updates STAT register and updates stat line
    fn update_stat(&mut self) {
        let stat = self.stat & 0xFC;
        self.stat = stat | self.mode;
        
        if self.lyc == self.ly {
            self.stat |= 0x04;
        } else {
            self.stat &= 0xFB;
        }

        let old_stat_line = self.stat_line;

        self.stat_line = (self.lyc == self.ly && self.stat & 0x40 != 0) |
                         (self.mode == 0      && self.stat & 0x20 != 0) |
                         (self.mode == 1      && self.stat & 0x10 != 0) |
                         (self.mode == 2      && self.stat & 0x08 != 0);

        self.stat_triggered = !old_stat_line && self.stat_line
    }

    fn calc_mode_3_dots(&self) -> u32 {
        let mut res = MODE_3_MIN_DOTS + (self.scx % 8) as u32;

        if self.win_enabled() && self.wy_cond {
            res += 6;
        }

        for i in &self.obj_buffer {
            let x = self.oam[*i][1];
            let offset = if self.win_enabled() && self.wy_cond && x + 7 <= self.wx { 
                0xFF - self.wx 
            } else { 
                self.scx 
            };
            
            res += 11 - min(5, (x + offset) % 8) as u32;
        }

        res
    }

    fn apply_palette(colour_id: &u8, palette: &u8) -> u8 {
        let id = colour_id << 1;
        (palette & (0x03 << id)) >> id
    }

    fn lcd_ppu_disabled(&self) -> bool {
        self.lcdc & 0x80 == 0
    }

    fn win_enabled(&self) -> bool {
        self.lcdc & 0x20 != 0 && self.lcdc & 0x01 != 0
    }

    fn obj_enabled(&self) -> bool {
        self.lcdc & 0x02 != 0
    }

    /// Returns true if bg and window will become blank and only objects are displayed on LCD
    fn obj_only(&self) -> bool {
        self.lcdc & 0x01 == 0
    }

    fn obj_size(&self) -> u8 {
        if self.lcdc & 0x04 == 0 { 8 } else { 16 }
    }

    fn reset_lcd(&mut self) {
        self.frame_buffer = [[0; LCD_WIDTH]; LCD_HEIGHT];
    }

    pub fn read_vram(&self, addr: usize) -> u8 {
        // Let cpu read from vram EVEN DURING MODE 3
        match addr {
            0x8000..=0x97FF => {
                let index = addr - 0x8000;
                self.tile_data[index / TILE_SIZE][index % TILE_SIZE]
            },
            0x9800..=0x9BFF => self.tile_map0[addr - 0x9800],
            0x9C00..=0x9FFF => self.tile_map1[addr - 0x9C00],
            _ => unreachable!(),
        }
    }

    pub fn write_vram(&mut self, addr: usize, byte: u8) {
        // Let cpu write to vram EVEN DURING MODE 3
        match addr {
            0x8000..=0x97FF => {
                let index = addr - 0x8000;
                self.tile_data[index / TILE_SIZE][index % TILE_SIZE] = byte;
            }
            0x9800..=0x9BFF => self.tile_map0[addr - 0x9800] = byte,
            0x9C00..=0x9FFF => self.tile_map1[addr - 0x9C00] = byte,
            _ => unreachable!(),
        }
    }

    pub fn read_oam(&self, addr: usize) -> u8 {
        if (self.mode != 3 && self.mode != 2) || self.lcd_ppu_disabled() {
            let index = addr - 0xFE00;
            self.oam[index / OAM_ENTRY_SIZE][index % OAM_ENTRY_SIZE]
        } else { 
            0xFF
        } 
    }

    pub fn write_oam(&mut self, addr: usize, byte: u8) {
        if (self.mode != 3 && self.mode != 2) || self.lcd_ppu_disabled() {
            let index = addr - 0xFE00;
            self.oam[index / OAM_ENTRY_SIZE][index % OAM_ENTRY_SIZE] = byte;
        }
    }

    pub fn read_lcdc(&self) -> u8 {
        self.lcdc
    }

    pub fn write_lcdc(&mut self, byte: u8) {
        if self.lcdc & 0x80 == 0 && byte & 0x80 != 0 {
            self.reset_lcd();   
        }
        self.lcdc = byte; 
    }

    pub fn read_stat(&self) -> u8 {
        if self.lcd_ppu_disabled() {
            self.stat & 0xFC
        } else {
            self.stat
        }
    }

    /// Bottom three bits are read-only.
    pub fn write_stat(&mut self, byte: u8) {
        let stat = self.stat & 0x07;
        self.stat = (byte & 0xF8) | stat;
    }

    pub fn read_scy(&self) -> u8 {
        self.scy
    }

    pub fn write_scy(&mut self, byte: u8) {
        self.scy = byte
    }

    pub fn read_scx(&self) -> u8 {
        self.scx
    }

    pub fn write_scx(&mut self, byte: u8) {
        self.scx = byte
    }

    pub fn read_ly(&self) -> u8 {
        if self.lcd_ppu_disabled() {
            0
        } else {
            self.ly
        }
    }

    pub fn read_lyc(&self) -> u8 {
        self.lyc
    }

    pub fn write_lyc(&mut self, byte: u8) {
        self.lyc = byte
    }

    pub fn read_dma(&self) -> u8 {
        self.dma
    }

    pub fn write_dma(&mut self, byte: u8) {
        self.dma = byte
    }

    pub fn read_bgp(&self) -> u8 { 
        self.bgp
    }

    pub fn write_bgp(&mut self, byte: u8) { 
        self.bgp = byte 
    }

    pub fn read_obp0(&self) -> u8 { 
        self.obp0 
    }

    pub fn write_obp0(&mut self, byte: u8) { 
        self.obp0 = byte 
    }

    pub fn read_obp1(&self) -> u8 { 
        self.obp1 
    }

    pub fn write_obp1(&mut self, byte: u8) { 
        self.obp1 = byte 
    }

    pub fn read_wy(&self) -> u8 { 
        self.wy 
    }

    pub fn write_wy(&mut self, byte: u8) { 
        self.wy = byte 
    }

    pub fn read_wx(&self) -> u8 { 
        self.wx 
    }

    pub fn write_wx(&mut self, byte: u8) { 
        self.wx = byte 
    }
    
}

#[cfg(test)]
mod tests {
    use crate::cpu::Cpu;

    use super::{LCD_HEIGHT, LCD_WIDTH};

    const TEST_FILE: &str = "roms/dmg-acid2.gb";
    const CHECKSUM: u32 = 203719462;

    #[test]
    fn ppu_test() {
        let mut cpu = Cpu::new(0x01B0, 0x0013, 0x00D8, 0x014D, 0x0100, 0xFFFE);
        cpu.bus.memory.load_from_file(TEST_FILE);

        let mut cycles: u32 = 0;

        while cycles < 5000000 {
            cycles += cpu.step() as u32;
        } 

        let mut sum: u32 = 0;
        
        for y in 0..LCD_HEIGHT {
            for x in 0..LCD_WIDTH {
                sum = sum.wrapping_add((cpu.bus.ppu.frame_buffer[y][x] as u32).wrapping_mul((x + LCD_WIDTH * y) as u32));
            }
        }

        assert!(sum == CHECKSUM, "checksum mismatch: got {} but expected {}", sum, CHECKSUM);
    }
}