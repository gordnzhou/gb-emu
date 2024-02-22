const ROM_SIZE: usize = 0x8000;
const ERAM_SIZE: usize = 0x2000;

pub struct Rom {
    rom: [u8; ROM_SIZE],
    eram: [u8; ERAM_SIZE],
}

impl Rom {
    pub fn new() -> Self {
        Rom { 
            rom: [0; ROM_SIZE],
            eram: [0; ERAM_SIZE],
        }
    }
}