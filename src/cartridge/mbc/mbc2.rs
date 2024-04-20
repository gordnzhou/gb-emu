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
    pub fn new_with_ram(rom: Vec<[u8; ROM_BANK_SIZE]>, rom_banks: usize) -> Self {     
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
    pub fn with_battery(mut self, battery: Battery) -> Self {
        match battery.load_ram() {
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
                self.rom[self.current_rom_bank][addr - 0x4000]
            }
            _ => unreachable!()
        }
    }

    fn write_rom(&mut self, addr: usize, byte: u8) {
        match addr {
            0x0000..=0x3FFF => {
                if addr & 0x100 == 0 {
                    self.ram_enabled = (byte & 0xF) == 0xA;
                } else {
                    let byte = byte + (byte & 0b1111 == 0) as u8;
                    let mask = self.rom_banks - 1;
                    self.current_rom_bank = byte as usize & mask;
                }
            },
            _ => {}
        }
    }

    fn read_ram(&self, addr: usize) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }
        0xF0 | self.ram[(addr - RAM_START) & 0b111111111] & 0xF
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
        battery.save_ram(&ram);
    }

    #[cfg(target_arch = "wasm32")]
    fn load_save(&mut self, data: Vec<u8>, save_type: &str) {
        assert!(save_type == "ram");

        let ram = Battery::parse_ram(data);
        for i in 0..MBC2_RAM_SIZE {
            self.ram[i] = ram[0][i] & 0xF;
        }
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
    use crate::cpu::test_helpers::test_mooneye_rom;
    use crate::cpu::GBModel::DMG;
    
    #[test]
    fn mbc2_bits_test() {
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc2/bits_romb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc2/bits_ramg.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc2/bits_unused.gb", DMG);
    }

    #[test]
    fn mbc2_rom_test() {
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc2/rom_1Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc2/rom_2Mb.gb", DMG);
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc2/rom_512kb.gb", DMG);
    }

    #[test]
    fn mbc2_ram_test() {
        test_mooneye_rom("roms/tests/mooneye/emulator-only/mbc2/ram.gb", DMG);
    }
}