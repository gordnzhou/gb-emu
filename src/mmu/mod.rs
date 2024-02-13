

const MEMORY_SIZE: usize = 0x10000;

pub struct MMU {
    memory: [u8; MEMORY_SIZE]
}

impl MMU {
    pub fn new() -> Self {
        MMU {
            memory: [0; MEMORY_SIZE]
        }
    }

    pub fn read_byte(&self, addr: usize) -> u8 {
        // FOR TESTING
        if addr == 0xFF44 {
            return 0x90;
        }

        // println!("reading byte at memory address: {:#06x}", addr);
        self.memory[addr]
    }

    pub fn read_word(&self, addr: usize) -> u16{
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn write_byte(&mut self, addr: usize, byte: u8) {
        // println!("writing memory at address: {:#06x} with byte: {:#04x}", addr, byte);
        self.memory[addr] = byte;
    }

    pub fn write_word(&mut self, addr: usize, word: u16) {
        self.write_byte(addr, word as u8);
        self.write_byte(addr.wrapping_add(1), (word >> 8) as u8);
    }
}