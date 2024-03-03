use std::fs::File;
use std::io::{self, Read};

const BOOTROM_PATH: &str = "roms/bootrom";

const ROM_SIZE: usize = 0x8000;
const ERAM_SIZE: usize = 0x2000;
const BOOTROM_SIZE: usize = 0x100;

pub struct Memory {
    rom: [u8; ROM_SIZE],
    eram: [u8; ERAM_SIZE],
    bootrom: [u8; BOOTROM_SIZE],
    bank: u8,
}

// TODO: add MBC support
impl Memory {
    pub fn new() -> Self {
        Memory { 
            rom: [0; ROM_SIZE],
            eram: [0; ERAM_SIZE],
            bootrom: [0; BOOTROM_SIZE],
            bank: 1,
        }
    }

    /// Writes to BANK register, which unmaps the boot ROM.
    pub fn write_bank(&mut self, byte: u8) {
        self.bank = byte;
    }

    pub fn read_bank(&self) -> u8 {
        self.bank
    }
    
    pub fn load_from_file(&mut self, rom_path: &str) {
        match Memory::read_from_file(rom_path) {
            Ok(rom_data) => {
                for i in 0..rom_data.len() {
                    self.rom[i] = rom_data[i];
                }

                println!("Sucessfully read ROM of size {} bytes.", rom_data.len());
                println!("First bytes: {:?}", &rom_data[..16]);
            }
            Err(err) => {
                eprintln!("Error reading ROM file: {}", err);
            }
        }
    }

    /// loads in boot ROM, setting BANK register to 0
    pub fn load_bootrom(&mut self) {
        self.bank = 0;

        match Memory::read_from_file(BOOTROM_PATH) {
            Ok(rom_data) => {
                for i in 0..BOOTROM_SIZE {
                    self.bootrom[i] = rom_data[i];
                }

                println!("Sucessfully read boot ROM.");
            }
            Err(err) => {
                eprintln!("Error reading boot ROM: {}", err);
            }
        }
    }

    fn read_from_file(file_path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(file_path)?;

        let mut rom_data = Vec::new();
        file.read_to_end(&mut rom_data)?;

        Ok(rom_data)
    }

    pub fn read_rom(&self, addr: usize) -> u8 {
        if addr < BOOTROM_SIZE && self.bank == 0 {
            self.bootrom[addr]
        } else {
            self.rom[addr]
        }
    }

    pub fn read_eram(&self, addr: usize) -> u8 {
        self.eram[addr - 0xA000]
    }
}