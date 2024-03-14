use std::fs::{read, write};

use super::mbc::RAM_BANK_SIZE;

pub const SAVE_PATH: &str = "saves/";

/// Saves and loads RAM state to file named after game title
pub struct Battery {
    file_location: String,
}

impl Battery {
    pub fn new(title: String) -> (Self, Option<Vec<[u8; RAM_BANK_SIZE]>>) {
        let file_location = format!("{}{}", SAVE_PATH, title);
        let ram = Battery::load_ram_from_file(&file_location);
        let battery = Battery { 
            file_location,
        };

        (battery, ram)
    }

    pub fn save_ram_to_file(&self, ram: &Vec<[u8; RAM_BANK_SIZE]>) {
        let ram_flat: Vec<u8> = ram.iter().flatten().copied().collect();

        match write(&self.file_location, ram_flat) {
            Ok(_) => println!("Saved RAM to: {}", self.file_location),
            Err(e) => println!("Unable to save RAM to {}: {}", self.file_location, e)
        }
    }

    fn load_ram_from_file(file_location: &str) -> Option<Vec<[u8; RAM_BANK_SIZE]>> {
        match read(file_location) {
            Ok(data) => {
                let ram = data.chunks_exact(RAM_BANK_SIZE).map(|chunk| {
                    let mut bank = [0; RAM_BANK_SIZE];
                    bank.copy_from_slice(chunk);
                    bank
                }).collect();
                println!("loaded RAM from {}", file_location);
                Some(ram)
            }
            Err(e) => {
                println!("Unable to load RAM from {}: {}", file_location, e);
                None
            }
        }
    }
}