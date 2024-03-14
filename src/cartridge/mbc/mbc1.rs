use std::cmp::{max, min};
use std::{fs::File, io::Read};
use std::io;

use crate::bus::RAM_START;
use crate::cartridge::battery::Battery;

use super::{Mbc, RAM_BANK_SIZE, ROM_BANK_SIZE};


pub struct Mbc1 {
    rom: Vec<[u8; ROM_BANK_SIZE]>,
    rom_banks: usize,
    ram: Option<Vec<[u8; RAM_BANK_SIZE]>>,
    ram_banks: usize,
    battery: Option<Battery>,
    current_rom_bank: usize,
    current_ram_bank: usize,
    ram_enabled: bool,
    banking_mode: bool,
}

impl Mbc for Mbc1 {
    fn read_rom(&self, addr: usize) -> u8 {
        match addr {
            0x0000..=0x3FFF => {
                let bank = if self.banking_mode { self.current_rom_bank & 0xE0 } else { 0 };
                self.rom[bank][addr]
            }
            0x4000..=0x7FFF => {
                let bank = match self.current_rom_bank {
                    0x20 | 0x40 | 0x60 => self.current_rom_bank + 1,
                    _ => self.current_rom_bank
                };

                self.rom[bank][addr - 0x4000]
            }
            _ => unreachable!()
        }
    }

    fn write_rom(&mut self, addr: usize, byte: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = (byte & 0xF) == 0xA,
            0x2000..=0x3FFF => {
                let mask = min(self.rom_banks as u8 - 1, 0b11111);
                self.current_rom_bank = max((byte & mask) as usize, 1)
            },
            0x4000..=0x5FFF => {
                if !self.banking_mode {
                    if self.ram_banks == 4 {
                        self.current_ram_bank = (byte & 3) as usize;
                    }
                } else {
                    if self.rom_banks >= 64 {
                        self.current_rom_bank = ((byte & 3) << 5) as usize | (self.current_rom_bank & 0x1F);
                    }
                }
            },
            0x6000..=0x7FFF => if self.ram_banks > 1 && self.rom_banks > 32 {
                self.banking_mode = byte != 0
            },
            _ => unreachable!()
        }
    }

    fn read_ram(&self, addr: usize) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        let ram_bank = if self.banking_mode { self.current_ram_bank } else { 0 };

        match &self.ram {
            Some(ram) => ram[ram_bank][addr - RAM_START],
            None => 0xFF
        }
    }

    fn write_ram(&mut self, addr: usize, byte: u8) {
        if !self.ram_enabled {
            return;
        }

        match &mut self.ram {
            Some(ram) => ram[self.current_ram_bank][addr - RAM_START] = byte,
            None => {}
        };
    }

    fn display(&self) -> String {
        let mut ret = format!("Mbc1 w/ {} ROM banks", self.rom_banks);
        if self.ram.is_some() {
            ret.push_str(&format!(" + {} RAM banks", self.ram_banks));
        }
        if self.battery.is_some() {
            ret.push_str(" + Battery");
        }
        ret
    }
}

impl Mbc1 {
    pub fn new(rom_path: &str, rom_banks: usize) -> Self {
        let rom = match Mbc1::read_from_file(rom_path, rom_banks) {
            Ok(rom) => rom,
            Err(err) => {
                panic!("Error reading ROM from {}: {}", rom_path, err);
            }
        };
        
        Mbc1 {
            rom,
            rom_banks,
            ram: None,
            ram_banks: 0,
            battery: None,
            current_rom_bank: 1,
            current_ram_bank: 0,
            ram_enabled: false,
            banking_mode: false,
        }
    }

    fn read_from_file(rom_path: &str, rom_banks: usize) -> io::Result<Vec<[u8; ROM_BANK_SIZE]>> {
        let mut file = File::open(rom_path)?;

        let mut rom_data = Vec::new();
        file.read_to_end(&mut rom_data)?;

        let mut rom = vec![[0; ROM_BANK_SIZE]; rom_banks];
        for i in 0..rom_data.len() {
            rom[i / ROM_BANK_SIZE][i % ROM_BANK_SIZE] = rom_data[i];
        }

        Ok(rom)
    }

    pub fn with_ram(mut self, ram_banks: usize) -> Self {
        self.ram = Some(vec![[0; RAM_BANK_SIZE]; ram_banks]);
        self.ram_banks = ram_banks;
        self
    }

    pub fn with_battery(mut self, title: String) -> Self {
        self.battery = Some(Battery::new(title));
        self
    }
}