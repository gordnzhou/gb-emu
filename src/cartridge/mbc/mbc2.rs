use crate::bus::RAM_START;
use crate::cartridge::battery::Battery;

use super::{Mbc, RAM_BANK_SIZE, ROM_BANK_SIZE};

const MBC2_RAM_SIZE: usize = 512;

pub struct Mbc2 {
    rom: Vec<[u8; ROM_BANK_SIZE]>,
    rom_banks: usize,
    ram: [u8; MBC2_RAM_SIZE],
    battery: Option<Battery>,
    current_rom_bank: usize,
    ram_enabled: bool,
}

impl Mbc2 {
    pub fn new_with_ram(rom_path: &str, rom_banks: usize) -> Self {
        let rom = match super::read_rom_from_file(rom_path, rom_banks) {
            Ok(rom) => rom,
            Err(err) => {
                panic!("Error reading ROM from {}: {}", rom_path, err);
            }
        };
        
        Mbc2 {
            rom,
            rom_banks,
            ram: [0; MBC2_RAM_SIZE],
            battery: None,
            current_rom_bank: 1,
            ram_enabled: false,
        }
    }

    /// Specifies battery and loads last RAM save (if any exists), otherwise creates a new one.
    pub fn with_battery(mut self, title: String) -> Self {
        let battery = Battery::new(title);  
        match battery.load_ram_from_file() {
            Some(ram) => {
                for i in 0..MBC2_RAM_SIZE {
                    self.ram[i] = ram[0][i] & 0xF;
                }
            },
            None => {}
        };
        self.battery = Some(battery);
        self
    }
}

impl Mbc for Mbc2 {
    fn read_rom(&self, addr: usize) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[0][addr],
            0x4000..=0x7FFF => {
                let bank = match self.current_rom_bank {
                    0x00 | 0x20 | 0x40 | 0x60 => self.current_rom_bank + 1,
                    _ => self.current_rom_bank
                };
                self.rom[bank][addr - 0x4000]
            }
            _ => unreachable!()
        }
    }

    fn write_rom(&mut self, addr: usize, byte: u8) {
        match addr {
            0x0000..=0x3FFF => {
                if (addr & 0x100) == 0 {
                    self.ram_enabled = (byte & 0xF) == 0xA
                } else {
                    self.current_rom_bank = byte as usize & 0xF;
                }
            },
            _ => {}
        }
    }

    fn read_ram(&self, addr: usize) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }
        self.ram[(addr - RAM_START) & 0b111111111] & 0xF
    }

    fn write_ram(&mut self, addr: usize, byte: u8) {
        if !self.ram_enabled {
            return;
        }
        self.ram[(addr - RAM_START) & 0b111111111] = byte & 0xF;
    }

    fn display(&self) -> String {
        let mut ret = format!("Mbc2 w/ {} ROM banks", self.rom_banks);
        if self.battery.is_some() {
            ret.push_str(" + Battery");
        }
        ret
    }

    fn save_state(&self) {
        let battery = match &self.battery {
            Some(battery) => battery,
            None => return
        };

        let mut ram = vec![[0; RAM_BANK_SIZE]; 1];
        for i in 0..MBC2_RAM_SIZE {
            ram[0][i] = self.ram[i] & 0xF
        }
        battery.save_ram_to_file(&ram);
    }
}