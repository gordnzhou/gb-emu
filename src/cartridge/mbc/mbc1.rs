use std::cmp::min;

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

impl Mbc1 {
    pub fn new(rom: Vec<[u8; ROM_BANK_SIZE]>, rom_banks: usize) -> Self {  
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

    /// Specifies RAM (sets ram to not None).
    pub fn with_ram(mut self, ram_banks: usize) -> Self {
        self.ram = Some(vec![[0; RAM_BANK_SIZE]; ram_banks]);
        self.ram_banks = ram_banks;
        self
    }

    /// ASSUMES: RAM has already be set to not None
    /// Specifies battery and loads last RAM save (if any exists), otherwise creates a new one.
    pub fn with_battery(mut self, battery: Battery) -> Self { 
        self.ram = Some(match battery.load_ram() {
            Some(ram) => {
                assert!(ram.len() == self.ram_banks, "Invalid RAM Save Size!");
                ram
            },
            None => vec![[0; RAM_BANK_SIZE]; self.ram_banks],
        });
        self.battery = Some(battery);
        self
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, addr: usize) -> u8 {
        match addr {
            0x0000..=0x3FFF => {
                let rom_bank = if self.banking_mode {
                    self.current_rom_bank & 0b1100000
                } else {
                    0
                };
                self.rom[rom_bank][addr]
            },
            0x4000..=0x7FFF => {
                self.rom[self.current_rom_bank][addr - 0x4000]
            }
            _ => unreachable!()
        }
    }

    fn write_rom(&mut self, addr: usize, byte: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = (byte & 0xF) == 0xA,
            0x2000..=0x3FFF => {
                let byte = byte + (byte & 0b11111 == 0) as u8;
                let mask = min(self.rom_banks as u8 - 1, 0b11111);
                self.current_rom_bank = (byte & mask) as usize;
            },
            0x4000..=0x5FFF => {
                if self.ram_banks > 1 {
                    self.current_ram_bank = (byte & (self.ram_banks as u8 - 1)) as usize;
                }

                if self.rom_banks > 0b11111 {
                    let mask = ((self.rom_banks as u8 - 1) - 0b11111) >> 5;
                    self.current_rom_bank = ((byte & mask) << 5) as usize | (self.current_rom_bank & 0b11111);
                } 
            },
            0x6000..=0x7FFF => if self.ram_banks > 1 || self.rom_banks > 32 {
                self.banking_mode = (byte & 1) != 0
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

        let ram_bank = if self.banking_mode { self.current_ram_bank } else { 0 };
        
        match &mut self.ram {
            Some(ram) => ram[ram_bank][addr - RAM_START] = byte,
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

    fn save_state(&self) {
        let battery = match &self.battery {
            Some(battery) => battery,
            None => return
        };

        let ram: &Vec<[u8; 8192]> = match &self.ram {
            Some(ram) => ram,
            None => return
        };

        battery.save_ram(ram);
    }

    #[cfg(target_arch = "wasm32")]
    fn load_save(&mut self, data: Vec<u8>, save_type: &str) {
        assert!(save_type == "ram");
        self.ram = Some(Battery::parse_ram(data));
    }

    #[cfg(target_arch = "wasm32")]
    fn save_id(&self) -> Option<String> {
        match &self.battery {
            Some(battery) => Some(battery.save_id()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::{test_helpers::test_mooneye_rom, GBModel::DMG};
    
    #[test]
    fn mbc1_bits_test() {
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/bits_bank1.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/bits_bank2.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/bits_mode.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/bits_ramg.gb", DMG);
    }

    #[test]
    fn mbc1_rom_test() {
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/rom_1Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/rom_2Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/rom_4Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/rom_8Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/rom_16Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/rom_512kb.gb", DMG);
    }

    #[test]
    fn mbc1_ram_test() {
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/ram_64kb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc1/ram_256kb.gb", DMG);
    }
}