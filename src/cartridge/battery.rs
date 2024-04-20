#[cfg(not(target_arch = "wasm32"))]
use std::fs::{create_dir_all, read, write};


#[cfg(target_arch = "wasm32")]
use {
    serde_wasm_bindgen::{from_value, to_value},
    crate::{save_to_db, load_from_db, log},
};

use super::{mbc::RAM_BANK_SIZE, rtc::{Rtc, RTC_REGISTERS_SIZE}};
pub const SAVE_PATH: &str = "saves";

/// Saves and loads RAM and/or RTC state to a file; identified by cartridge header title.
#[cfg(not(target_arch = "wasm32"))]
pub struct Battery {
    save_folder: String,
    ram_file_location: String,
    rtc_file_location: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl Battery {
    pub fn new(id_name: String) -> Self {
        let save_folder: String = format!("{}/{}", SAVE_PATH, id_name);
        let ram_file_location = format!("{}/ram", save_folder);
        let rtc_file_location = format!("{}/rtc", save_folder);

        Battery { 
            save_folder, 
            ram_file_location, 
            rtc_file_location,
        }
    }

    /// Saves current RAM state.
    pub fn save_ram(&self, ram: &Vec<[u8; RAM_BANK_SIZE]>) {
        if let Err(e) = create_dir_all(&self.save_folder) {
            println!("Failed to create directory: {}", e);
        }

        let ram_flat: Vec<u8> = ram.iter().flatten().copied().collect();

        match write(&self.ram_file_location, ram_flat) {
            Ok(_) => println!("Saved RAM to: {}", self.ram_file_location),
            Err(e) => println!("Unable to save RAM to {}: {}", self.ram_file_location, e)
        }
    }

    /// Loads RAM from last save and returns it or returns None is no valid save found.
    pub fn load_ram(&self) -> Option<Vec<[u8; RAM_BANK_SIZE]>> {
        match read(&self.ram_file_location) {
            Ok(data) => {
                println!("loaded RAM from {}", self.ram_file_location);
                Some(Battery::parse_ram(data))
            }
            Err(_) => {
                println!("No RAM save detected...");
                None
            }
        }
    }

    /// Saves current RTC state.
    pub fn save_rtc(&self, rtc: &Rtc) {
        if let Err(e) = create_dir_all(&self.save_folder) {
            println!("Failed to create directory: {}", e);
        }

        match write(&self.rtc_file_location, rtc.to_save()) {
            Ok(_) => println!("Saved RTC state to: {}", self.ram_file_location),
            Err(e) => println!("Unable to save RTC state to {}: {}", self.ram_file_location, e)
        }
    }

    /// Loads RTC from last save and returns it or returns None is no valid save found.
    pub fn load_rtc(&self) -> Option<Rtc> {
        match read(&self.rtc_file_location) {
            Ok(data) => {
                println!("loaded RTC state from {}", self.ram_file_location);
                Some(Battery::parse_rtc(data))
            }
            Err(_) => {
                println!("No RTC save detected...");
                None
            }
        }
    }
}

impl Battery {
    pub fn parse_ram(data: Vec<u8>) -> Vec<[u8; RAM_BANK_SIZE]> {
        data.chunks_exact(RAM_BANK_SIZE).map(|chunk| {
            let mut bank = [0; RAM_BANK_SIZE];
            bank.copy_from_slice(chunk);
            bank
        }).collect()
    }

    pub fn parse_rtc(data: Vec<u8>) -> Rtc {
        let mut registers = [0; RTC_REGISTERS_SIZE + 8];
        registers.copy_from_slice(&data[0..RTC_REGISTERS_SIZE + 8]);
        Rtc::from_save(registers)
    }
}

#[cfg(target_arch = "wasm32")]
pub struct Battery {
    save_id: String,
}

#[cfg(target_arch = "wasm32")]
impl Battery {
    pub fn new(save_id: String) -> Self {
        Battery { 
            save_id,
        }
    }

    pub fn save_ram(&self, ram: &Vec<[u8; RAM_BANK_SIZE]>) {
        let ram_flat: Vec<u8> = ram.iter().flatten().copied().collect();
        save_to_db(&self.save_id, "ram", to_value(&ram_flat).unwrap());
    }

    pub fn load_ram(&self) -> Option<Vec<[u8; RAM_BANK_SIZE]>> {
        load_from_db(&self.save_id, "ram");
        None
    }

    pub fn save_rtc(&self, rtc: &Rtc) {
        save_to_db(&self.save_id, "rtc", to_value(&rtc.to_save()).unwrap())
    }

    pub fn load_rtc(&self) -> Option<Rtc> {
        load_from_db(&self.save_id, "rtc");
        None
    }

    pub fn save_id(&self) -> String {
        self.save_id.clone()
    }
}