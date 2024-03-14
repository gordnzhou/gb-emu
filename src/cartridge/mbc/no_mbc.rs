use crate::{bus::{RAM_START, ROM_START}, cartridge::Cartridge};
use super::{Mbc, RAM_MEMORY_SPACE, ROM_MEMORY_SPACE};


pub struct NoMbc {
    rom: [u8; ROM_MEMORY_SPACE],
    ram: [u8; RAM_MEMORY_SPACE],
}

impl Mbc for NoMbc {
    fn read_rom(&self, addr: usize) -> u8 {
        self.rom[addr - ROM_START]
    }

    fn write_rom(&mut self, _addr: usize, _byte: u8) {
        // do nothing
    }

    fn read_ram(&self, addr: usize) -> u8 {
        self.ram[addr - RAM_START]
    }

    fn write_ram(&mut self, addr: usize, byte: u8) {
        self.ram[addr - RAM_START] = byte;
    }

    fn display(&self) -> String {
        String::from("No Mbc")
    }

    fn save_ram(&self) {
        // do nothing
    }
}

impl NoMbc {
    pub fn new(rom_path: &str) -> Self {
        let mut rom = [0; ROM_MEMORY_SPACE];
        match Cartridge::read_from_file(rom_path) {
            Ok(rom_data) => {
                for i in 0..rom_data.len() {
                    rom[i] = rom_data[i];
                }
            }
            Err(err) => {
                panic!("Error reading ROM from {}: {}", rom_path, err);
            }
        }

        NoMbc { 
            rom,
            ram: [0; RAM_MEMORY_SPACE],
        }
    }
}