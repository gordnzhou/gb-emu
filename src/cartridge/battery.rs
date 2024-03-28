use std::fs::{create_dir_all, read, write};

use super::{mbc::RAM_BANK_SIZE, rtc::{Rtc, RTC_REGISTERS_SIZE}};

pub const SAVE_PATH: &str = "saves";

/// Saves and loads RAM and/or RTC state to a file; identified by cartridge header title.
pub struct Battery {
    rom_file_location: String,
    rtc_file_location: String,
}

impl Battery {
    pub fn new(file_name: String) -> Self {
        let save_folder: String = format!("{}/{}", SAVE_PATH, file_name);
        let rom_file_location = format!("{}/ram", save_folder);
        let rtc_file_location = format!("{}/rtc", save_folder);

        if let Err(e) = create_dir_all(&save_folder) {
            println!("Failed to create directory: {}", e);
        }

        Battery { rom_file_location, rtc_file_location }
    }

    /// Saves current RAM to file. 
    pub fn save_ram_to_file(&self, ram: &Vec<[u8; RAM_BANK_SIZE]>) {
        let ram_flat: Vec<u8> = ram.iter().flatten().copied().collect();

        match write(&self.rom_file_location, ram_flat) {
            Ok(_) => println!("Saved RAM to: {}", self.rom_file_location),
            Err(e) => println!("Unable to save RAM to {}: {}", self.rom_file_location, e)
        }
    }

    /// Loads RAM from save file and returns it or returns None is no valid save found.
    pub fn load_ram_from_file(&self) -> Option<Vec<[u8; RAM_BANK_SIZE]>> {
        match read(&self.rom_file_location) {
            Ok(data) => {
                let ram = data.chunks_exact(RAM_BANK_SIZE).map(|chunk| {
                    let mut bank = [0; RAM_BANK_SIZE];
                    bank.copy_from_slice(chunk);
                    bank
                }).collect();
                println!("loaded RAM from {}", self.rom_file_location);
                Some(ram)
            }
            Err(_) => {
                println!("No RAM save detected...");
                None
            }
        }
    }

    /// Saves current RTC to file.
    pub fn save_rtc_to_file(&self, rtc: &Rtc) {
        match write(&self.rtc_file_location, rtc.to_save()) {
            Ok(_) => println!("Saved RTC state to: {}", self.rom_file_location),
            Err(e) => println!("Unable to save RTC state to {}: {}", self.rom_file_location, e)
        }
    }

    /// Loads RTC from save file and returns it or returns None is no valid save found.
    pub fn load_rtc_from_file(&self) -> Option<Rtc> {
        match read(&self.rtc_file_location) {
            Ok(data) => {
                let mut registers = [0; RTC_REGISTERS_SIZE + 8];
                registers.copy_from_slice(&data[0..RTC_REGISTERS_SIZE + 8]);
                println!("loaded RTC state from {}", self.rom_file_location);
                Some(Rtc::from_save(registers))
            }
            Err(_) => {
                println!("No RTC save detected...");
                None
            }
        }
    }
}