use std::cmp::max;
use crate::bus::RAM_START;
use crate::cartridge::battery::Battery;
use crate::cartridge::rtc::Rtc;

use super::{Mbc, RAM_BANK_SIZE, ROM_BANK_SIZE};

pub struct Mbc3 {
    rom: Vec<[u8; ROM_BANK_SIZE]>,
    rom_banks: usize,
    ram: Option<Vec<[u8; RAM_BANK_SIZE]>>,
    ram_banks: usize,
    rtc: Option<Rtc>,
    battery: Option<Battery>,
    current_rom_bank: usize,
    current_ram_bank: usize,
    ram_rtc_enabled: bool,
    prev_latch_write: u8,
    using_ram: bool,
}

impl Mbc3 {
    pub fn new(rom: Vec<[u8; ROM_BANK_SIZE]>, rom_banks: usize) -> Self {    
        Mbc3 {
            rom,
            rom_banks,
            ram: None,
            ram_banks: 0,
            battery: None,
            rtc: None,
            current_rom_bank: 1,
            current_ram_bank: 0,
            ram_rtc_enabled: false,
            prev_latch_write: 0xFF,
            using_ram: true,
        }
    }

    /// Specifies RTC registers (sets timer to not None).
    pub fn with_rtctimer(mut self) -> Self {
        self.rtc = Some(Rtc::new());
        self
    }

    /// Specifies RAM (sets ram to not None).
    pub fn with_ram(mut self, ram_banks: usize) -> Self {
        self.ram = Some(vec![[0; RAM_BANK_SIZE]; ram_banks]);
        self.ram_banks = ram_banks;
        self
    }

    /// Specifies Battery and loads in existing RAM (if ram is not None), 
    /// and RTC registers (if timer is not None).
    pub fn with_battery(mut self, battery: Battery) -> Self {   
        if self.ram.is_some() {
            match battery.load_ram() {
                Some(ram) => {
                    assert!(ram.len() == self.ram_banks, "Invalid RAM Save Size!");
                    self.ram = Some(ram)
                },
                None => {}
            };
        }
        if self.rtc.is_some() {
            match battery.load_rtc() {
                Some(rtc) => self.rtc = Some(rtc),
                None => {}
            }
        }
        self.battery = Some(battery);
        self
    }
}

impl Mbc for Mbc3 {
    fn read_rom(&self, addr: usize) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[0][addr],
            0x4000..=0x7FFF => {
                let mask = self.rom_banks - 1;
                let bank = max(self.current_rom_bank & mask, 1);
                self.rom[bank][addr - 0x4000]
            }
            _ => unreachable!()
        }
    }

    fn write_rom(&mut self, addr: usize, byte: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_rtc_enabled = (byte & 0xF) == 0xA,
            0x2000..=0x3FFF => self.current_rom_bank = max((byte & 0b1111111) as usize, 1),
            0x4000..=0x5FFF => {
                match byte {
                    0x00..=0x03 => {
                        self.using_ram = true;
                        self.current_ram_bank = byte as usize
                    }
                    0x08..=0x0C => {
                        self.using_ram = false;
                        match &mut self.rtc {
                            Some(rtc) => rtc.set_active_reg(byte),
                            None => {}
                        };
                    }
                    _ => {}
                }
            },
            0x6000..=0x7FFF => {
                if self.prev_latch_write == 0 && byte == 1 {
                    match &mut self.rtc {
                        Some(rtc) => rtc.latch_clock_data(),
                        None => {}
                    };
                }
                self.prev_latch_write = byte;
            },
            _ => unreachable!()
        }
    }

    fn read_ram(&self, addr: usize) -> u8 {
        if !self.ram_rtc_enabled {
            return 0xFF;
        }

        if self.using_ram {
            match &self.ram {
                Some(ram) => ram[self.current_ram_bank][addr - RAM_START],
                None => 0xFF
            }
        } else {
            match &self.rtc {
                Some(rtc) => rtc.read(),
                None => 0xFF
            }
        }
    }

    fn write_ram(&mut self, addr: usize, byte: u8) {
        if !self.ram_rtc_enabled {
            return;
        }

        if self.using_ram {
            match &mut self.ram {
                Some(ram) => ram[self.current_ram_bank][addr - RAM_START] = byte,
                None => {}
            };
        } else {
            match &mut self.rtc {
                Some(rtc) => rtc.write(byte),
                None => {}
            }
        }
    }

    fn display(&self) -> String {
        let mut ret = format!("Mbc3 w/ {} ROM banks", self.rom_banks);
        if self.rtc.is_some() {
            ret.push_str(" + RTC Timer");
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

        match &self.ram {
            Some(ram) => battery.save_ram(ram),
            None => {}
        };

        match &self.rtc {
            Some(rtc) => battery.save_rtc(rtc),
            None => {}
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn load_save(&mut self, data: Vec<u8>, save_type: &str) {
        if save_type == "ram" {
            self.ram = Some(Battery::parse_ram(data));
        } else {
            self.rtc = Some(Battery::parse_rtc(data));
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