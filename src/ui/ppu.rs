use crate::cpu::Interrupt;

use std::cmp::{min, max};

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
    mode: u8,
    mode_elapsed_dots: u32,

    mode_3_dots: u32,
    cur_pixel_x: usize,
    win_in_frame: bool,

    obj_buffer_index: usize,
    obj_buffer: Vec<usize>,
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
            mode: 2,
            mode_elapsed_dots: 0,

            mode_3_dots: 0,
            cur_pixel_x: 0,
            win_in_frame: false,

            obj_buffer_index: 0,
            obj_buffer: Vec::new(),
        }
    }

    /// TODO: Steps through the PPU and Display and returns all interrupts triggered.
    pub fn step(&mut self, cycles: u8) -> Vec<Interrupt> {
        self.entered_vblank = false;

        if self.lcdc & 0x80 == 0 {
            return Vec::new();
        }

        let mut interrupts = Vec::new();

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
            self.step_mode(mode_end - self.mode_elapsed_dots);

            // TODO: edge case when mode 3 transitions to mode 2 but mode 2 gets extended 
            if self.next_mode() {
                interrupts.push(Interrupt::VBlank);
            }

            self.step_mode(next_dots - mode_end);
            self.mode_elapsed_dots = next_dots - mode_end;
        }

        if self.update_stat() {
            interrupts.push(Interrupt::Stat);
        }

        interrupts
    }

    //. Transitions to next mode in PPU; returns true if next mode is 1 (VBlank).
    fn next_mode(&mut self) -> bool {
        self.mode = match self.mode {
            0 => {
                self.cur_pixel_x = 0;
                self.obj_buffer = Vec::new();
                self.obj_buffer_index = 0;

                if self.ly + 1 == LCD_HEIGHT as u8 {
                    // HBlank -> VBlank
                    self.ly = 0;
                    self.win_in_frame = false;
                    self.entered_vblank = true;
                    1
                } else {
                    // HBlank -> OAM Search
                    self.ly += 1;
                    self.win_in_frame = self.wy == self.ly;
                    2
                }
            },
            1 => {
                // VBlank -> OAM Search
                2 
            },
            2 => {
                // OAM Search -> Drawing
                self.mode_3_dots = self.calc_mode_end_3();
                3
            },
            3 => {
                // Drawing -> HBlank
                0
            },
            _ => unreachable!()
        };

        self.mode == 1
    }

    /// ASSUME: mode_elapsed_dots + dots is not greater than duration of current mode.
    /// Steps through period (in dots) in current mode.
    fn step_mode(&mut self, dots: u32) {
        if dots == 0 {
            return;
        }

        if self.mode == 2 {
            // check an attribute from OAM every 2 dotsx
            let mut fetches = (dots + 1) / 2;

            while fetches > 0 && self.obj_buffer_index < OAM_SIZE && self.obj_buffer.len() == 10 {
                let y = self.oam[self.obj_buffer_index];
                
                if self.ly + 16 >= y && self.ly + 16 < y + self.obj_size()  {
                    self.obj_buffer.push(self.obj_buffer_index)
                }

                self.obj_buffer_index += 4;
                fetches -= 1;
            }

        } else if self.mode == 3 {
            let mut pixels_left = dots;

            // TODO: implement BG and OAM FIFO 
            while self.cur_pixel_x < LCD_WIDTH && pixels_left > 0 {
                
                self.cur_pixel_x += 1;
                pixels_left -= 1;
            }
        }     
        //      if lcdc.0 is set clear bg and window
        //      else
        //          draw bg pixels from tilemap (in lcdc.3) using tile data (lcdc.4), at scx, scy with colour bgp
        //          if enabled (lcdc.5), draw window pixels from tilemap (in lcdc.6) using tile data (lcdc.4) at wx, wy with colour bgp
        // 
        //     draw objects on top of bg/windows always at OAM x, y and other attributes from tile set 0x8000 
        //          ignore obj pixels with id 00 (transparent)
        //          extend mode 3 dot length based on OBJ penalty algorithm
        //
        //     mode 3 can be 172 - 289 dots

        // ** up to TWO modes can be be run in one step**
    }

    /// updates STAT register and returns true if stat interrupt was triggered
    // TODO: implemen stat blocking (update interrupt line struct field instead of returning stat)
    fn update_stat(&mut self) -> bool {
        let stat = self.stat & 0xFC;
        self.stat = stat | self.mode;
        
        if self.lyc == self.ly {
            self.stat |= 0x04;
        } else {
            self.stat &= 0xFB;
        }

        (self.lyc == self.ly && self.stat & 0x40 != 0) |
        (self.mode == 0      && self.stat & 0x20 != 0) |
        (self.mode == 1      && self.stat & 0x10 != 0) |
        (self.mode == 2      && self.stat & 0x08 != 0)
    }

    fn calc_mode_end_3(&self) -> u32 {
        let mut res = MODE_3_MIN_DOTS + (self.scx % 8) as u32;

        if self.win_enabled() && self.win_in_frame {
            res += 6;
        }

        for i in &self.obj_buffer {
            let x = self.oam[*i];
            let offset = if self.win_enabled() && self.win_in_frame && self.wx + 7 <= self.wx{ 
                0xFF - self.wx 
            } else { 
                self.scx 
            };
            
            res +=  11 - min(5, (x + offset) % 8) as u32;
        }

        res
    }

    fn render_pixel(&mut self, x: usize, y: usize, colour: u8) {
        self.frame_buffer[y][x] = colour;
    }

    fn lcd_ppu_enabled(&self) -> bool {
        self.lcdc & 0x80 != 0
    }

    fn win_enabled(&self) -> bool {
        self.lcdc & 0x20 != 0
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

    fn win_tmap_start(&self) -> usize {
        if self.lcdc & 0x08 == 0 { 0x1800 } else { 0x1C00 }
    }

    fn bg_tmap_start(&self) -> usize {
        if self.lcdc & 0x40 == 0 { 0x1800 } else { 0x1C00 }
    }

    /// Returns address of given tile id in vram (for bg and window only).
    fn get_bgwin_tile_data(&self, tile_id: u8) -> usize {
        if self.lcdc & 0x10 == 1 { 
            0x1000 + 16 * tile_id as usize
        } else {
            (0x1800 + (16 * (tile_id as i8) as i32)) as usize
        }
    }

    pub fn read_vram(&self, addr: usize) -> u8 {
        if self.mode != 3 || self.lcdc & 0x80 == 0 {
            self.vram[addr - 0x8000]
        } else {
            0xFF
        }

    }

    pub fn write_vram(&mut self, addr: usize, byte: u8) {
        if self.mode != 3 || self.lcdc & 0x80 == 0 { 
            self.vram[addr - 0x8000] = byte;
        }
    }

    pub fn read_oam(&self, addr: usize) -> u8 {
        if self.mode != 3 && self.mode != 2 || self.lcdc & 0x80 == 0 {
            self.oam[addr - 0xFE00]
        } else { 
            0xFF
        }
        
    }

    pub fn write_oam(&mut self, addr: usize, byte: u8) {
        if self.mode != 3 && self.mode != 2 || self.lcdc & 0x80 == 0 {
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