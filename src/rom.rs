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

    pub fn read_rom(&self, addr: usize) -> u8 {
        self.rom[addr]
    }

    pub fn write_rom(&mut self, addr: usize, byte: u8) {
        self.rom[addr] = byte;
    }

    pub fn read_eram(&self, addr: usize) -> u8 {
        self.eram[addr - 0xA000]
    }

    pub fn write_eram(&mut self, addr: usize, byte: u8) {
        self.eram[addr - 0xA000] = byte;
    }
}