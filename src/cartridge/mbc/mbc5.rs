use crate::bus::RAM_START;
use crate::cartridge::battery::Battery;

use super::{Mbc, RAM_BANK_SIZE, ROM_BANK_SIZE};

pub struct Mbc5 {
    rom: Vec<[u8; ROM_BANK_SIZE]>,
    rom_banks: usize,
    ram: Option<Vec<[u8; RAM_BANK_SIZE]>>,
    ram_banks: usize,
    battery: Option<Battery>,
    current_rom_bank: usize,
    current_ram_bank: usize,
    ram_enabled: bool,
    rumble: bool,
}

impl Mbc5 {
    pub fn new(rom_path: &str, rom_banks: usize) -> Self {
        let rom = match super::read_rom_from_file(rom_path, rom_banks) {
            Ok(rom) => rom,
            Err(err) => {
                panic!("Error reading ROM from {}: {}", rom_path, err);
            }
        };
        
        Mbc5 {
            rom,
            rom_banks,
            ram: None,
            ram_banks: 0,
            battery: None,
            current_rom_bank: 1,
            current_ram_bank: 0,
            ram_enabled: false,
            rumble: false,
        }
    }

    pub fn with_rumble(mut self) -> Self {
        // Can't think of a way to implement this???
        self.rumble = true;
        self
    }

    /// Specifies RAM (sets ram to not None).
    pub fn with_ram(mut self, ram_banks: usize) -> Self {
        self.ram = Some(vec![[0; RAM_BANK_SIZE]; ram_banks]);
        self.ram_banks = ram_banks;
        self
    }

    /// ASSUMES: RAM has already be set to not None
    /// Specifies battery and loads last RAM save (if any exists), otherwise creates a new one.
    pub fn with_battery(mut self, title: String) -> Self {
        let battery = Battery::new(title);  
        self.ram = Some(match battery.load_ram_from_file() {
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

impl Mbc for Mbc5 {
    fn read_rom(&self, addr: usize) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[0][addr],
            0x4000..=0x7FFF => self.rom[self.current_rom_bank & (self.rom_banks - 1)][addr - 0x4000],
            _ => unreachable!()
        }
    }

    fn write_rom(&mut self, addr: usize, byte: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = (byte & 0xF) == 0xA,
            0x2000..=0x2FFF => self.current_rom_bank = byte as usize,
            0x3000..=0x3FFF => self.current_rom_bank |= (byte as usize & 1) << 8,
            0x4000..=0x5FFF => if (byte as usize) < self.ram_banks {
                self.current_ram_bank = byte as usize;
            },
            _ => {}
        }
    }

    fn read_ram(&self, addr: usize) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }
        match &self.ram {
            Some(ram) => ram[self.current_ram_bank][addr - RAM_START],
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
        let mut ret = format!("Mbc5 w/ {} ROM banks", self.rom_banks);
        if self.rumble {
            ret.push_str(&format!(" + Rumble (not supported)"));
        }
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

        battery.save_ram_to_file(ram);
    }
}

#[cfg(test)]
mod tests {
    use crate::test_mooneye_rom;
    use crate::cpu::GBModel::DMG;
    

    #[test]
    fn mbc5_rom_test() {
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc5/rom_1Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc5/rom_2Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc5/rom_4Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc5/rom_8Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc5/rom_16Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc5/rom_32Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc5/rom_64Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc5/rom_512kb.gb", DMG);
    }
}