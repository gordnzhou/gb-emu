use std::cmp::min;

use crate::{cpu::GBModel, emulator::{BYTES_PER_PIXEL, LCD_BYTE_WIDTH}};

const TILE_SIZE: usize = 16;
const TILE_ENTRIES: usize = 384;

const OAM_ENTRY_SIZE: usize = 4;
const OAM_ENTRIES: usize = 40;

const TILE_MAP_SIZE: usize = 0x0400;

// 8 palettes * 4 colours/palette * 2 bytes/colour 
const CRAM_SIZE: usize = 64;

const LCD_WIDTH: usize= 160;
const LCD_HEIGHT: usize = 144;

const SCAN_LINE_DOTS: u32 = 456;
const MODE_1_DOTS: u32 = SCAN_LINE_DOTS * 10;
const MODE_2_DOTS: u32 = 80;
const MODE_3_MIN_DOTS: u32 = 172;

pub const COLOURS: [[u8; BYTES_PER_PIXEL]; 4] = [
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
    model: GBModel,
    tile_data0: [[u8; TILE_SIZE]; TILE_ENTRIES],
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

    // frame buffer representing the LCD screen
    pub frame_buffer: [u8; LCD_BYTE_WIDTH * LCD_HEIGHT],
    pub stat_triggered: bool,
    pub entered_vblank: bool,

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
    obj_buffer: Vec<OAMEntry>,
    last_vblank_scanline: u32,

    // CGB_ONLY
    vbk: u8,
    bgpi: u8,
    obpi: u8,
    opri: u8,
    tile_data1: [[u8; TILE_SIZE]; TILE_ENTRIES],
    attr_map0: [u8; TILE_MAP_SIZE],
    attr_map1: [u8; TILE_MAP_SIZE],
    cram_bg: [u8; CRAM_SIZE],
    cram_obj: [u8; CRAM_SIZE],

    // for HBlank DMA transfer (CGB only)
    entered_hblank: bool,
}

impl Ppu {
    pub fn new(model: GBModel) -> Self {
        Ppu { 
            model,
            tile_data0: [[0; TILE_SIZE]; TILE_ENTRIES],
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
            entered_vblank: false,
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
            last_vblank_scanline: 0,

            vbk: 0,
            bgpi: 0,
            obpi: 0,
            opri: 0,
            tile_data1: [[0; TILE_SIZE]; TILE_ENTRIES],
            attr_map0: [0; TILE_MAP_SIZE],
            attr_map1: [0; TILE_MAP_SIZE],
            cram_bg: [0; CRAM_SIZE],
            cram_obj: [0; CRAM_SIZE],
            entered_hblank: false,
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
                if !self.is_cgb() || self.opri != 0 {
                    self.obj_buffer.sort_by(|a, b| { a.x.cmp(&b.x)});
                }
                self.mode_3_dots = self.calc_mode_3_dots();
                Mode::Drawing3
            },
            Mode::Drawing3 => {
                self.entered_hblank = true;
                Mode::HBlank0
            },
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
                        self.obj_buffer.push(OAMEntry::new(self.oam[self.obj_buffer_index]));
                    }
                    self.obj_buffer_index += 1;
                    fetches -= 1;
                }
            }
            Mode::Drawing3 => {
                let mut pixels_left = dots;
                while self.cur_pixel_x < LCD_WIDTH && pixels_left > 0 {
                    self.wy_cond |= self.wy == self.ly;
                    self.wx_cond = self.wx as usize <= self.cur_pixel_x + 7;

                    // future TODO (maybe): implement BG and OAM FIFO 
                    let colour = self.render_pixel(self.cur_pixel_x, self.ly as usize); 
                    let display_colour = match self.model {
                        GBModel::DMG => COLOURS[colour as usize],
                        GBModel::CGB => Ppu::rgb555_to_argb8888(colour),
                    };

                    for i in 0..BYTES_PER_PIXEL {
                        self.frame_buffer[usize::from(self.ly) * LCD_BYTE_WIDTH
                            + usize::from(self.cur_pixel_x) * BYTES_PER_PIXEL
                            + i] = display_colour[i];
                    }
        
                    self.cur_pixel_x += 1;
                    pixels_left -= 1;
                }
            }
            _ => {}
        }

        self.update_stat();
    }

    fn render_pixel(&mut self, lcd_x: usize, lcd_y: usize) -> u16 {
        let (mut colour, bg_priority, bg_is_0) = self.render_bgwin_pixel(lcd_x, lcd_y);
        colour = self.render_obj_pixel(colour, lcd_x, lcd_y, bg_priority, bg_is_0);
        colour
    }

    fn render_bgwin_pixel(&mut self, lcd_x: usize, lcd_y: usize) -> (u16, bool, bool) {
        let mut is_bg = true;
        let mut x = (lcd_x + self.scx as usize) % 0xFF;
        let mut y = (lcd_y as usize + self.scy as usize) % 0xFF;

        if self.win_enabled() && self.wx_cond && self.wy_cond {
            self.line_has_window = true;
            is_bg = false;
            x = lcd_x + 7 - self.wx as usize;
            y = self.win_counter;       
        }

        let tmap_addr = (x >> 3) + ((y >> 3) << 5);
        let tile_id = self.fetch_bgwin_tile_id(tmap_addr, is_bg);

        match self.model {
            GBModel::DMG => {
                if self.lcdc & 0x01 == 0 {
                    return (0, false, false);
                }

                let colour_id = self.fetch_tile_pixel(self.lcdc & 0x10 == 0, tile_id, x, y, false, false, false);
                (Ppu::apply_palette_dmg(&colour_id, &self.bgp), false, false)
            }
            GBModel::CGB => {
                let attributes = self.fetch_bgwin_attribute(tmap_addr, is_bg);
                let palette = attributes & 0x07;
                let bank = attributes & 0x08 != 0;
                let x_flip = attributes & 0x20 != 0;
                let y_flip = attributes & 0x40 != 0;
                let priority = attributes & 0x80 != 0;

                let colour_id = self.fetch_tile_pixel(self.lcdc & 0x10 == 0, tile_id, x, y, x_flip, y_flip, bank);
                (Ppu::apply_palette_cgb(&colour_id, self.cram_bg, &palette), priority, colour_id == 0)
            }
        }

    }

    fn render_obj_pixel(&self, bg_colour: u16, lcd_x: usize, lcd_y: usize, bg_priority: bool, bg_is_0: bool) -> u16 {
        let mut colour = bg_colour;

        for obj in &self.obj_buffer {
            if !self.obj_enabled() {
                break;
            }

            if !(obj.x <= lcd_x + 8 && lcd_x < obj.x) {
                continue;
            }

            let tile_id = obj.fetch_tile_id(lcd_y, self.obj_size());
            let tile_x = lcd_x + 8 - obj.x;
            let tile_y = lcd_y as usize + 16 - obj.y;
            let use_bank_1 = obj.cgb_use_bank_1 && matches!(self.model, GBModel::CGB);
            let id = self.fetch_tile_pixel(false, tile_id as u8, tile_x, tile_y, 
                obj.x_flip, obj.y_flip, use_bank_1);

            if id != 0 { 
                match self.model {
                    GBModel::DMG => {
                        if colour == 0 || obj.has_priority {
                            let palette = if !obj.dmg_palette { self.obp0 } else { self.obp1 };
                            colour = Ppu::apply_palette_dmg(&id, &palette);
                        }
                    }
                    GBModel::CGB => {
                        if bg_is_0 || self.lcdc & 0x01 == 0 || (obj.has_priority && !bg_priority) {
                            colour = Ppu::apply_palette_cgb(&id, self.cram_obj, &obj.cgb_palette)
                        }
                    }
                }
                break;
            }
        }

        colour
    }

    /// Returns colour id from tile_id's tile at (tile_x, tile_y).
    /// Set addr_mode to false for 0x8000 tile_data addressing, true for 0x8800 addressing.
    fn fetch_tile_pixel(&self, addr_mode: bool, tile_id: u8, tile_x: usize, tile_y: usize, x_flip: bool, y_flip: bool, bank: bool) -> u8 {
        let mut tile_data = &self.tile_data0;
        if bank && matches!(self.model, GBModel::CGB) {
            tile_data = &self.tile_data1;
        }
        
        let tile = tile_data[if !addr_mode { 
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

    fn fetch_bgwin_tile_id(&self, tmap_addr: usize, is_bg: bool) -> u8 {     
        let lcd_bit = if is_bg { 0x08 } else { 0x40 };
        if self.lcdc & lcd_bit == 0 { 
            self.tile_map0[tmap_addr] 
        } else { 
            self.tile_map1[tmap_addr] 
        }
    }

    fn fetch_bgwin_attribute(&self, tmap_addr: usize, is_bg: bool) -> u8 {     
        let lcd_bit = if is_bg { 0x08 } else { 0x40 };
        if self.lcdc & lcd_bit == 0 { 
            self.attr_map0[tmap_addr] 
        } else { 
            self.attr_map1[tmap_addr] 
        }
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
        if matches!(self.model, GBModel::DMG) && self.lcdc & 0x01 == 0 {
            false
        } else {
            self.lcdc & 0x20 != 0
        }
    }

    fn obj_enabled(&self) -> bool {
        self.lcdc & 0x02 != 0
    }

    fn obj_size(&self) -> u8 {
        if self.lcdc & 0x04 == 0 { 8 } else { 16 }
    }

    fn reset_lcd(&mut self) {
        self.frame_buffer = [0; LCD_BYTE_WIDTH * LCD_HEIGHT];
    }

    pub fn read_vram(&self, addr: usize) -> u8 {
        // Let cpu read from vram EVEN DURING MODE 3
        let mut tile_data = &self.tile_data0;
        let mut map0= &self.tile_map0;
        let mut map1 = &self.tile_map1;

        if self.vbk > 0 && matches!(self.model, GBModel::CGB) {
            tile_data = &self.tile_data1;
            map0 = &self.attr_map0;
            map1 = &self.attr_map1;
        }

        match addr {
            0x8000..=0x97FF => {
                let index = addr - 0x8000;
                tile_data[index / TILE_SIZE][index % TILE_SIZE]
            },
            0x9800..=0x9BFF => map0[addr - 0x9800],
            0x9C00..=0x9FFF => map1[addr - 0x9C00],
            _ => unreachable!(),
        }
    }

    pub fn write_vram(&mut self, addr: usize, byte: u8) {
        // Let cpu write from vram EVEN DURING MODE 3
        let mut tile_data = &mut self.tile_data0;
        let mut map0= &mut self.tile_map0;
        let mut map1 = &mut self.tile_map1;

        if self.vbk > 0 && matches!(self.model, GBModel::CGB) {
            tile_data = &mut self.tile_data1;
            map0 = &mut self.attr_map0;
            map1 = &mut self.attr_map1;
        }

        // Let cpu write to vram EVEN DURING MODE 3
        match addr {
            0x8000..=0x97FF => {
                let index = addr - 0x8000;
                tile_data[index / TILE_SIZE][index % TILE_SIZE] = byte;
            }
            0x9800..=0x9BFF => map0[addr - 0x9800] = byte,
            0x9C00..=0x9FFF => map1[addr - 0x9C00] = byte,
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

    pub fn read_io(&self, addr: usize) -> u8 {
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

            0xFF4F => self.vbk & 0xFE,
            0xFF68 => self.bgpi,
            0xFF69 => self.cram_bg[(self.bgpi & 0x3F) as usize],
            0xFF6A => self.obpi,
            0xFF6B => self.cram_obj[(self.obpi & 0x3F) as usize],
            0xFF6C => self.opri,
            
            _ => unreachable!()
        }
    }

    /// Writes to PPU registers; returns true if DMA transfer is triggered.
    pub fn write_io(&mut self, addr: usize, byte: u8) {
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

            0xFF4F => self.vbk = byte,
            0xFF68 => self.bgpi = byte,
            0xFF69 => {
                self.cram_bg[(self.bgpi & 0x3F) as usize] = byte;
                if self.bgpi & 0x80 != 0 {
                    self.bgpi += 1;
                    self.bgpi &= 0b10111111;
                }
            },
            0xFF6A => self.obpi = byte,
            0xFF6B => {
                self.cram_obj[(self.obpi & 0x3F) as usize] = byte;
                if self.obpi & 0x80 != 0 {
                    self.obpi += 1;
                    self.obpi &= 0b10111111;
                }
            },
            0xFF6C => self.opri = byte,
            _ => unreachable!()
        };
    }

    pub fn write_dma(&mut self, byte: u8) {
        self.dma = byte;
    }

    /// signals if PPU has just entered HBlank
    pub fn entered_hblank(&mut self) -> bool {
        let ret = self.entered_hblank;
        self.entered_hblank = false;
        ret
    }

    fn calc_mode_3_dots(&self) -> u32 {
        let mut res = MODE_3_MIN_DOTS + (self.scx % 8) as u32;

        if self.win_enabled() && self.wy_cond {
            res += 6;
        }

        for obj in &self.obj_buffer {
            let offset = if self.win_enabled() && self.wy_cond && (obj.x as u8) + 7 <= self.wx { 
                0xFF - self.wx 
            } else { 
                self.scx 
            };
            
            res += 11 - min(5, (obj.x as u16 + offset as u16) % 8) as u32;
        }

        res
    }

    /// Returns index of colour in COLOURS (0-3)
    fn apply_palette_dmg(colour_id: &u8, palette: &u8) -> u16 {
        let id = colour_id << 1;
        ((palette & (0x03 << id)) >> id) as u16
    }

    /// Returns RGB555 representation of colour
    fn apply_palette_cgb(colour_id: &u8, cram: [u8; CRAM_SIZE], palette_id: &u8) -> u16 {
        let index_0 = (*palette_id << 3) as usize + (*colour_id << 1) as usize;
        let index_1 = index_0 + 1;
        cram[index_0] as u16 | ((cram[index_1] as u16) << 8)
    }

    fn rgb555_to_argb8888(color: u16) -> [u8; BYTES_PER_PIXEL] {
        let r5 = (color >> 0) & 0x1F;
        let g5 = (color >> 5) & 0x1F;
        let b5 = (color >> 10) & 0x1F;
     
        let r8 = (r5 * 255 + 15) / 31;
        let g8 = (g5 * 255 + 15) / 31;
        let b8 = (b5 * 255 + 15) / 31;
    
        [b8 as u8, g8 as u8, r8 as u8, 0xFF]
    }

    fn mode_to_num(mode: &Mode) -> u8 {
        match mode {
            Mode::HBlank0 => 0,
            Mode::VBlank1 => 1,
            Mode::OamScan2 => 2,
            Mode::Drawing3 => 3,
        }
    } 

    fn is_cgb(&mut self) -> bool {
        matches!(self.model, GBModel::CGB)
    }
}

struct OAMEntry {
    y: usize,
    x: usize, 
    tile_id: usize,
    cgb_palette: u8,
    cgb_use_bank_1: bool,
    dmg_palette: bool,
    x_flip: bool,
    y_flip: bool,
    has_priority: bool,
}

impl OAMEntry {
    fn new(data: [u8; OAM_ENTRY_SIZE]) -> Self {
        let attributes = data[3];

        OAMEntry {
            y: data[0] as usize,
            x: data[1] as usize,
            tile_id: data[2] as usize,
            cgb_palette: attributes & 0x07,
            cgb_use_bank_1: attributes & 0x08 != 0,
            dmg_palette: attributes & 0x10 != 0,
            x_flip: attributes & 0x20 != 0,
            y_flip: attributes & 0x40 != 0,
            has_priority: attributes & 0x80 == 0,
        }
    }

    /// Calculates appriate tile id based on current y pos,
    /// and if objects are 8 or 16 pixels tall.
    fn fetch_tile_id(&self, lcd_y: usize, obj_size: u8) -> usize {
        if obj_size == 16 {
            if (lcd_y as usize + 8 >= self.y) ^ self.y_flip {
                self.tile_id | 0x01
            } else {
                self.tile_id & 0xFE
            }
        } else {
            self.tile_id
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{cartridge::Cartridge, cpu::Cpu};

    const DMG_ACID: &str = "roms/tests/dmg-acid2.gb";
    const DMG_CHECKHASH: u64 = 7936837427979048709;

    const CGB_ACID: &str = "roms/tests/cgb-acid2.gbc";
    const CGB_CHECKHASH: u64 = 10090950622310532208;

    #[test]
    fn ppu_dmg_test() {
        let cartridge = Cartridge::from_file(DMG_ACID, false);
        let mut cpu = Cpu::new(cartridge, crate::cpu::GBModel::DMG);
        let mut cycles: u32 = 0;
        while cycles < 5000000 {
            cycles += cpu.step() as u32;
        } 
        
        let hash = fnv1a(&cpu.bus.ppu.frame_buffer);
        assert!(hash == DMG_CHECKHASH, "hash mismatch: got {} but expected {}", hash, DMG_CHECKHASH);
    }

    #[test]
    fn ppu_cgb_test() {
        let cartridge = Cartridge::from_file(CGB_ACID, false);
        let mut cpu = Cpu::new(cartridge, crate::cpu::GBModel::CGB);
        let mut cycles: u32 = 0;
        while cycles < 5000000 {
            cycles += cpu.step() as u32;
        } 
        
        let hash = fnv1a(&cpu.bus.ppu.frame_buffer);
        assert!(hash == CGB_CHECKHASH, "hash mismatch: got {} but expected {}", hash, CGB_CHECKHASH);
    }

    fn fnv1a(bytes: &[u8]) -> u64 {
        let mut hash = 0xcbf29ce484222325;
        for byte in bytes {
            hash = hash ^ (*byte as u64);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
}