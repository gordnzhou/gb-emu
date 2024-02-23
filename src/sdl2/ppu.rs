const VRAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0x00A0;

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
        }
    }

    pub fn step(&mut self) {
        
    }

    pub fn read_vram(&self, addr: usize) -> u8 {
        self.vram[addr - 0x8000]
    }

    pub fn write_vram(&mut self, addr: usize, byte: u8) {
        self.vram[addr - 0x8000] = byte;
    }

    pub fn read_oam(&self, addr: usize) -> u8 {
        self.oam[addr - 0xFE00]
    }

    pub fn write_oam(&mut self, addr: usize, byte: u8) {
        self.oam[addr - 0xFE00] = byte;
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

    /// Bottom two bits are read-only.
    pub fn write_stat(&mut self, byte: u8) {
        let stat = self.stat & 0x03;
        self.stat = (byte & 0xFC) | stat;
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

    /// Writes to DMA register then starts a DMA transfer of bytes from 0xNN00-0xNN9F
    /// to 0xFE00-0xFE9F (OAM) after current cpu instruction for 160 M-cycles (640 dots).
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