const VRAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0x00A0;

pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
}

impl Ppu {
    pub fn new() -> Self {
        Ppu { 
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
        }
    }
}