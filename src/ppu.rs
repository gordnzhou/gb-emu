use std::cmp::min;

use crate::emulator::{BYTES_PER_PIXEL, LCD_BYTE_WIDTH};

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

pub const COLORS: [[u8; 4]; 4] = [
    [0x0F, 0xBC, 0x9B, 0xFF], // #9BBC0F => white
    [0x0F, 0xAC, 0x8B, 0xFF], // #8BAC0F => light grey
    [0x30, 0x62, 0x30, 0xFF], // #306230 => dark grey
    [0x0F, 0x38, 0x0F, 0xFF], // #0F380F => black
];

#[derive(PartialEq)]
enum Mode {
    HBlank0, 
    VBlank1,
    OamScan2,
    Drawing3
}

pub struct Ppu {
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
    pub frame_buffer: [u8; LCD_WIDTH * LCD_HEIGHT * 4],
    pub stat_triggered: bool,
    stat_line: bool,
    mode: Mode,
    mode_elapsed_dots: u32,

    mode_3_dots: u32,
    cur_pixel_x: usize,
    wy_cond: bool,
    wx_cond: bool,
    line_has_window: bool,
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

            frame_buffer: [0; LCD_BYTE_WIDTH * LCD_HEIGHT],
            stat_triggered: false,
            stat_line: false,

            mode: Mode::VBlank1,
            mode_elapsed_dots: 0,

            mode_3_dots: 0,
            cur_pixel_x: 0,
            wy_cond: false,
            wx_cond: false,
            line_has_window: false,
            win_counter: 0,

            obj_buffer_index: 0,
            obj_buffer: Vec::new(),

            entered_vblank: false,
            last_vblank_scanline: 0,
        }
    }

    /// Steps through the PPU over the given period (in cycles).
    pub fn step(&mut self, cycles: u8) {
        if self.lcd_ppu_disabled() { return; }
        self.stat_triggered = false;

        let dots = cycles as u32 * 4;
        let next_dots = self.mode_elapsed_dots + dots;
        let mode_end = match self.mode {
            Mode::HBlank0 => SCAN_LINE_DOTS - self.mode_3_dots - MODE_2_DOTS,
            Mode::VBlank1 => MODE_1_DOTS,
            Mode::OamScan2 => MODE_2_DOTS,
            Mode::Drawing3 => self.mode_3_dots,
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

    // Updates PPU to next mode state.
    fn next_mode(&mut self) {
        self.mode = match self.mode {
            Mode::HBlank0 => {
                self.cur_pixel_x = 0;
                self.obj_buffer = Vec::new();
                self.obj_buffer_index = 0;
                self.ly += 1;

                if self.ly == LCD_HEIGHT as u8 {
                    self.wy_cond = false;
                    self.win_counter = 0;
                    self.entered_vblank = true;
                    self.last_vblank_scanline = 0;
                    Mode::VBlank1
                } else {        
                    self.win_counter += self.line_has_window as usize;
                    self.line_has_window = false;
                    Mode::OamScan2
                }
            },
            Mode::VBlank1 => {
                self.ly = 0;
                Mode::OamScan2
            },
            Mode::OamScan2 => {
                self.wx_cond = false;
                self.obj_buffer_index = 0;
                self.obj_buffer.sort_by(|a, b| { self.oam[*a][1].cmp(&self.oam[*b][1])});
                self.mode_3_dots = self.calc_mode_3_dots();
                Mode::Drawing3
            },
            Mode::Drawing3 => Mode::HBlank0,
        };
    }

    /// ASSUME: self.mode_elapsed_dots + dots will NOT exceed duration of current mode.
    /// Step through period (in dots) over the current mode (do nothing for mode 1 and 0).
    fn step_mode(&mut self, dots: u32) {
        if dots == 0 { return; }

        match self.mode {
            Mode::VBlank1 => {
                if self.last_vblank_scanline + dots >= SCAN_LINE_DOTS {
                    self.ly += 1;
                }
                self.last_vblank_scanline = (self.last_vblank_scanline + dots) % SCAN_LINE_DOTS;

                if self.ly == 153 && self.last_vblank_scanline >= 4 {
                    self.ly = 0;
                }
            }
            Mode::OamScan2 => {
                let mut fetches = (dots + 1) / 2;
                while fetches > 0 && self.obj_buffer_index < OAM_ENTRIES && self.obj_buffer.len() < 10 {
                    let obj_y = self.oam[self.obj_buffer_index][0];   
                    if self.ly + 16 >= obj_y && self.ly + 16 < obj_y + self.obj_size()  {
                        self.obj_buffer.push(self.obj_buffer_index);
                    }
                    self.obj_buffer_index += 1;
                    fetches -= 1;
                }
            }
            Mode::Drawing3 => {
                let mut pixels_left = dots;
                while self.cur_pixel_x < LCD_WIDTH && pixels_left > 0 {
                    self.wy_cond |= self.wy == self.ly;
                    self.wx_cond = if self.wx < 7 {
                        self.wx as usize <= self.cur_pixel_x + 7
                    } else {
                        self.wx as usize <= self.cur_pixel_x + 7
                    };

                    // future TODO (maybe): implement BG and OAM FIFO 
                    let colour = self.render_pixel(self.cur_pixel_x, self.ly as usize) as usize; 
                    for i in 0..=3 {
                        self.frame_buffer[usize::from(self.ly as usize) * LCD_BYTE_WIDTH
                            + usize::from(self.cur_pixel_x) * BYTES_PER_PIXEL
                            + i] = COLORS[colour][i];
                    }
                    
                    self.cur_pixel_x += 1;
                    pixels_left -= 1;
                }
            }
            _ => {}
        }

        self.update_stat();
    }

    fn render_pixel(&mut self, lcd_x: usize, lcd_y: usize) -> u8 {
        let mut colour = self.render_bgwin_pixel(lcd_x, lcd_y);
        colour = self.render_obj_pixel(colour, lcd_x, lcd_y);
        colour
    }

    fn render_bgwin_pixel(&mut self, lcd_x: usize, lcd_y: usize) -> u8 {
        if self.obj_only() {
            return 0;
        }

        let tile_data =  if self.win_enabled() && self.wx_cond && self.wy_cond {
            self.line_has_window = true;
            // assert!(lcd_x + 7 >= self.wx as usize, "{} {}", lcd_x, self.wx);
            let x = lcd_x + 7 - self.wx as usize;
            let y = self.win_counter;
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

                if attributes & 0x80 == 0 || colour == 0 {
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
        self.stat = stat | Ppu::mode_to_num(&self.mode);
        
        if self.lyc == self.ly {
            self.stat |= 0x04;
        } else {
            self.stat &= 0xFB;
        }

        let old_stat_line = self.stat_line;

        self.stat_line = (self.lyc == self.ly && self.stat & 0x40 != 0) |
            (self.mode == Mode::HBlank0 && self.stat & 0x20 != 0) |
            (self.mode == Mode::VBlank1 && self.stat & 0x10 != 0) |
            (self.mode == Mode::OamScan2 && self.stat & 0x08 != 0);

        self.stat_triggered = !old_stat_line && self.stat_line
    }

    fn lcd_ppu_disabled(&self) -> bool {
        self.lcdc & 0x80 == 0
    }

    fn win_enabled(&self) -> bool {
        if self.lcdc & 0x01 == 0 {
            false
        } else {
            self.lcdc & 0x20 != 0
        }
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
        self.frame_buffer = [0; LCD_BYTE_WIDTH * LCD_HEIGHT];
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
        if self.can_access_oam() {
            let index = addr - 0xFE00;
            self.oam[index / OAM_ENTRY_SIZE][index % OAM_ENTRY_SIZE]
        } else { 
            0xFF
        } 
    }

    pub fn write_oam(&mut self, addr: usize, byte: u8) {
        if self.can_access_oam() {
            let index = addr - 0xFE00;
            self.oam[index / OAM_ENTRY_SIZE][index % OAM_ENTRY_SIZE] = byte;
        }
    }

    fn can_access_oam(&self) -> bool {
        self.lcd_ppu_disabled() ||
        (self.mode != Mode::Drawing3 && self.mode != Mode::OamScan2)
    }

    pub fn read_io(&self, addr: usize,) -> u8 {
        match addr {
            0xFF40 => self.lcdc,
            0xFF41 => if self.lcd_ppu_disabled() { self.stat & 0xFC } else { self.stat },
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => if self.lcd_ppu_disabled() { 0 } else { self.ly },
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => unreachable!()
        }
    }

    /// Writes to PPU registers; returns true if DMA transfer is triggered.
    pub fn write_io(&mut self, addr: usize, byte: u8) -> bool {
        match addr {
            0xFF40 => {
                if self.lcdc & 0x80 == 0 && byte & 0x80 != 0 {
                    self.reset_lcd();   
                }
                self.lcdc = byte; 
            },
            0xFF41 => {
                let stat = self.stat & 0x07;
                self.stat = (byte & 0xF8) | stat;
            },
            0xFF42 => self.scy = byte,
            0xFF43 => self.scx = byte,
            0xFF44 => {},
            0xFF45 => self.lyc = byte,
            0xFF46 => self.dma = byte,
            0xFF47 => self.bgp = byte,
            0xFF48 => self.obp0 = byte,
            0xFF49 => self.obp1 = byte,
            0xFF4A => self.wy = byte,
            0xFF4B => self.wx = byte,
            _ => unreachable!()
        };

        addr == 0xFF46
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
            
            res += 11 - min(5, (x as u16 + offset as u16) % 8) as u32;
        }

        res
    }

    fn apply_palette(colour_id: &u8, palette: &u8) -> u8 {
        let id = colour_id << 1;
        (palette & (0x03 << id)) >> id
    }

    fn mode_to_num(mode: &Mode) -> u8 {
        match mode {
            Mode::HBlank0 => 0,
            Mode::VBlank1 => 1,
            Mode::OamScan2 => 2,
            Mode::Drawing3 => 3,
        }
    } 
}

#[cfg(test)]
mod tests {
    use crate::{cartridge::Cartridge, cpu::Cpu};

    use super::{LCD_HEIGHT, LCD_WIDTH};

    const TEST_FILE: &str = "roms/tests/dmg-acid2.gb";
    const CHECKSUM: u32 = 3249083280;

    #[test]
    fn ppu_test() {
        let cartridge = Cartridge::from_file(TEST_FILE, false);
        let mut cpu = Cpu::new(cartridge);

        let mut cycles: u32 = 0;

        while cycles < 5000000 {
            cycles += cpu.step() as u32;
        } 

        let mut sum: u32 = 0;
        
        for y in 0..LCD_HEIGHT {
            for x in 0..LCD_WIDTH {
                sum = sum.wrapping_add((cpu.bus.ppu.frame_buffer[x + LCD_WIDTH * y] as u32).wrapping_mul((x + LCD_WIDTH * y) as u32));
            }
        }

        assert!(sum == CHECKSUM, "checksum mismatch: got {} but expected {}", sum, CHECKSUM);
    }
}