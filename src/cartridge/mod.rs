mod mbc;
mod header;

use std::fs::File;
use std::io::{self, Read};

use self::header::Header;

const BOOTROM_PATH: &str = "roms/bootrom.gb";

const ROM_SIZE: usize = 0x8000;
const ERAM_SIZE: usize = 0x2000;
const BOOTROM_SIZE: usize = 0x100;

pub struct Cartridge {
    rom: [u8; ROM_SIZE],
    eram: [u8; ERAM_SIZE],
    bootrom: [u8; BOOTROM_SIZE],
    bank: u8,
    header: Header,
    with_bootrom: bool,
}

// TODO: add MBC support
impl Cartridge {
    pub fn from_file(rom_path: &str, with_bootrom: bool) -> Self {
        let mut rom = [0; ROM_SIZE];
        let mut bootrom = [0; BOOTROM_SIZE];
        let mut bank = 1;
        
        if with_bootrom {
            bank = 0;
            match Cartridge::read_from_file(BOOTROM_PATH) {
                Ok(rom_data) => {
                    for i in 0..rom_data.len() {
                        bootrom[i] = rom_data[i];
                    }
                }
                Err(err) => {
                    eprintln!("Error reading ROM file at {}: {}", rom_path, err);
                }
            }
        }

        // TODO: change this to support larger files
        match Cartridge::read_from_file(rom_path) {
            Ok(rom_data) => {
                for i in 0..rom_data.len() {
                    rom[i] = rom_data[i];
                }
            }
            Err(err) => {
                eprintln!("Error reading ROM file at {}: {}", rom_path, err);
            }
        }

        Cartridge { 
            rom,
            eram: [0; ERAM_SIZE],
            bootrom,
            bank,
            header: Header::from_bytes(rom[0x0100..0x0150].try_into().unwrap()),
            with_bootrom,
        }
    }

    pub fn has_bootrom(&self) -> bool {
        self.with_bootrom
    }

    pub fn get_title(&self) -> String {
        self.header.title.clone().replace("\0", "")
    }

    /// Writes to BANK register, which unmaps the boot ROM.
    pub fn write_bank(&mut self, byte: u8) {
        self.bank = byte;
    }

    pub fn read_bank(&self) -> u8 {
        self.bank
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