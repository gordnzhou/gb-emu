use std::cmp::min;

const VRAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0x00A0;

const LCD_WIDTH: usize= 160;
const LCD_HEIGHT: usize = 144;

const SCAN_LINE_DOTS: u32 = 456;
const MODE_1_DOTS: u32 = SCAN_LINE_DOTS * 10;
const MODE_2_DOTS: u32 = 80;
const MODE_3_MIN_DOTS: u32 = 172;

pub struct Ppu {
    // 16-byte types stored here
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
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
    pub entered_vblank: bool,
    pub stat_triggered: bool,
    stat_line: bool,
    mode: u8,
    mode_elapsed_dots: u32,

    mode_3_dots: u32,
    cur_pixel_x: usize,
    win_in_frame: bool,
    win_counter: u8,

    obj_buffer_index: usize,
    obj_buffer: Vec<usize>,

    count: u64,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu { 
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
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
            entered_vblank: false,
            stat_triggered: false,
            stat_line: false,

            mode: 2,
            mode_elapsed_dots: 0,

            mode_3_dots: 0,
            cur_pixel_x: 0,
            win_in_frame: false,
            win_counter: 0,

            obj_buffer_index: 0,
            obj_buffer: Vec::new(),

            count: 0,
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

    // Updates PPU to next mode state
    fn next_mode(&mut self) {
        self.mode = match self.mode {
            0 => {
                self.cur_pixel_x = 0;
                self.obj_buffer = Vec::new();
                self.obj_buffer_index = 0;

                if self.ly + 1 == LCD_HEIGHT as u8 {
                    // HBlank -> VBlank
                    self.ly = 0;
                    self.count += 1;
                    self.win_in_frame = false;
                    self.win_counter = 0;
                    self.entered_vblank = true;
                    1
                } else {
                    // HBlank -> OAM Search
                    self.ly += 1;
                    self.win_in_frame = self.wy == self.ly;
                    self.win_counter += if self.win_enabled() && self.win_in_frame { 1 } else { 0 };
                    2
                }
            },
            1 => {
                // VBlank -> OAM Search
                2 
            },
            2 => {
                // OAM Search -> Drawing
                self.obj_buffer_index = 0;
                self.obj_buffer.sort_by(|a, b| { self.oam[a + 1].cmp(&self.oam[b + 1])});
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

            while fetches > 0 && self.obj_buffer_index < OAM_SIZE && self.obj_buffer.len() < 10 {
                let y = self.oam[self.obj_buffer_index];
                
                if self.ly + 16 >= y && self.ly + 16 < y + self.obj_size()  {
                    self.obj_buffer.push(self.obj_buffer_index);
                }

                self.obj_buffer_index += 4;
                fetches -= 1;
            }

        } else if self.mode == 3 {
            let mut pixels_left = dots;

            // TODO: implement BG and OAM FIFO 
            while self.cur_pixel_x < LCD_WIDTH && pixels_left > 0 {
                let bg_win_colour = if !self.obj_only() {
                    if self.win_enabled() && self.win_in_frame {
                        let tile_pixel_x = self.cur_pixel_x + self.wx as usize - 7;
                        let tile_pixel_y = (self.win_counter + self.wy) as usize;
                        let win_tile_id = self.get_win_tile_id(tile_pixel_x / 8, tile_pixel_y / 8);
                        let win_tile_addr = self.get_bgwin_tile_data_addr(win_tile_id);
                        
                        let offset = 2 * (tile_pixel_y as usize % 8);
                        let pixel_lo = if self.vram[win_tile_addr + offset] & (1 << (tile_pixel_x % 8)) != 0 { 1 } else { 0 };
                        let pixel_hi = if self.vram[win_tile_addr + offset + 1] & (1 << (tile_pixel_x % 8)) != 0 { 1 } else { 0 };
                        self.bg_palette(pixel_hi << 1 | pixel_lo)
                    } else {
                        let tile_pixel_x = (self.cur_pixel_x + self.scx as usize) % 0xFF;
                        let tile_pixel_y = (self.ly as usize + self.scy as usize) % 0xFF;
                        let bg_tile_id = self.get_bg_tile_id(tile_pixel_x / 8, tile_pixel_y / 8);
                        let bg_tile_addr = self.get_bgwin_tile_data_addr(bg_tile_id);

                        let offset = 2 * (tile_pixel_y as usize % 8);
                        let pixel_lo = if self.vram[bg_tile_addr + offset] & (1 << (tile_pixel_x % 8)) != 0 { 1 } else { 0 };
                        let pixel_hi = if self.vram[bg_tile_addr + offset + 1] & (1 << (tile_pixel_x % 8)) != 0 { 1 } else { 0 };
                        self.bg_palette(pixel_hi << 1 | pixel_lo)
                    }
                } else { 0 };

                let obj_colour = if self.obj_enabled() && self.obj_buffer.len() > 0 {
                    println!("sprite");
                    // TODO: draw objects on top of bg/windows always at OAM x, y. flipx, flipy from tile set 0x8000 
                    //  priority, 
                    0
                } else { 0 };

                let colour = if obj_colour != 0 { obj_colour } else { bg_win_colour };
                self.frame_buffer[self.ly as usize][self.cur_pixel_x] = colour;
                self.cur_pixel_x += 1;
                pixels_left -= 1;
            }
        }     

        self.update_stat();
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

        if self.win_enabled() && self.win_in_frame {
            res += 6;
        }

        for i in &self.obj_buffer {
            let x = self.oam[*i];
            let offset = if self.win_enabled() && self.win_in_frame && x + 7 <= self.wx { 
                0xFF - self.wx 
            } else { 
                self.scx 
            };
            
            res += 11 - min(5, (x + offset) % 8) as u32;
        }

        res
    }

    fn bg_palette(&self, id: u8) -> u8 {
        let id = id << 1;
        (self.bgp & (0x03 << id)) >> id
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

    /// ASSUME: 0 <= tmap_x, tmap_y < 32
    /// Returns tile id at (tmap_x, tmap_y) from window tilemap.
    fn get_win_tile_id(&self, tmap_x: usize, tmap_y: usize) -> u8 {
        let start: usize = if self.lcdc & 0x40 == 0 { 0x1800 } else { 0x1C00 };
        self.vram[start + tmap_x + 32 * tmap_y]
    }

    // ASSUME: 0 <= tmap_x, tmap_y < 32
    /// Returns tile id at (tmap_x, tmap_y) from background tilemap.
    fn get_bg_tile_id(&self, tmap_x: usize, tmap_y: usize) -> u8 {
        let start: usize = if self.lcdc & 0x08 == 0 { 0x1800 } else { 0x1C00 };
        self.vram[start + tmap_x + 32 * tmap_y]
    }

    /// Returns start address of given tile id in vram (for bg and window only).
    fn get_bgwin_tile_data_addr(&self, tile_id: u8) -> usize {
        if self.lcdc & 0x10 == 1 { 
            0x0000 + 16 * tile_id as usize
        } else {
            (0x0800 + (16 * (tile_id as i8) as i32)) as usize
        }
    }

    pub fn read_vram(&self, addr: usize) -> u8 {
        if self.mode != 3 || self.lcd_ppu_disabled() {
            self.vram[addr - 0x8000]
        } else {
            0xFF
        }
    }

    pub fn write_vram(&mut self, addr: usize, byte: u8) {
        if self.mode != 3 || self.lcd_ppu_disabled() { 
            self.vram[addr - 0x8000] = byte;
        }
    }

    pub fn read_oam(&self, addr: usize) -> u8 {
        if (self.mode != 3 && self.mode != 2) || self.lcd_ppu_disabled() {
            self.oam[addr - 0xFE00]
        } else { 
            0xFF
        } 
    }

    pub fn write_oam(&mut self, addr: usize, byte: u8) {
        if (self.mode != 3 && self.mode != 2) || self.lcd_ppu_disabled() {
            self.oam[addr - 0xFE00] = byte;
        }
    }

    pub fn read_lcdc(&self) -> u8 {
        self.lcdc
    }

    pub fn write_lcdc(&mut self, byte: u8) {
        self.lcdc = byte; 
    }

    pub fn read_stat(&self) -> u8 {
        self.stat
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
        // FOR TESTING
        // return 0x90;
        
        self.ly
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
    use super::*;

    #[test]
    fn test_bfg_palette() {
       let mut ppu = Ppu::new();
        ppu.write_bgp(0b11001001);

        assert_eq!(ppu.bg_palette(0), 0b01);
        assert_eq!(ppu.bg_palette(1), 0b10);
        assert_eq!(ppu.bg_palette(2), 0b00);
        assert_eq!(ppu.bg_palette(3), 0b11);

        ppu.write_bgp(0b01110010);

        assert_eq!(ppu.bg_palette(0), 0b10);
        assert_eq!(ppu.bg_palette(1), 0b00);
        assert_eq!(ppu.bg_palette(2), 0b11);
        assert_eq!(ppu.bg_palette(3), 0b01);
    }
}