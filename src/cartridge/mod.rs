mod mbc;
mod header;
mod battery;
mod rtc;

use std::fs::File;
use std::io::{self, Read};

use self::header::Header;
use self::mbc::Mbc;

const BOOTROM_PATH: &str = "roms/bootrom.gb";
const BOOTROM_SIZE: usize = 0x100;

pub struct Cartridge {
    bootrom: [u8; BOOTROM_SIZE],
    bank: u8,
    header: Header,
    with_bootrom: bool,
    mbc: Box<dyn Mbc>,
}

impl Cartridge {
    /// Loads cartridge from the given file path (and optionally runs it with boot ROM).
    pub fn from_file(rom_path: &str, with_bootrom: bool) -> Self {
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
                    eprintln!("Error reading bootrom file from {}: {}", BOOTROM_PATH, err);
                }
            }
        }

        let header = match Header::from_file(rom_path) {
            Ok(header) => header,
            Err(err) => {
                panic!("Error reading header from {}: {}", rom_path, err);
            }
        };

        let mbc = mbc::make_mbc(rom_path, &header);
        println!("Detected MBC: {}", mbc.display());

        Cartridge { 
            bootrom,
            bank,
            header,
            with_bootrom,
            mbc,
        }
    }

    pub fn has_bootrom(&self) -> bool {
        self.with_bootrom
    }

    pub fn cgb_compatible(&self) -> bool {
        self.header.cgb_compatible()
    }

    pub fn get_title(&self) -> String {
        self.header.title()
    }

    /// Writes to BANK register, which unmaps the boot ROM.
    pub fn write_bank(&mut self, byte: u8) {
        self.bank = byte;
    }

    pub fn read_bank(&self) -> u8 {
        self.bank
    }

    pub fn read_from_file(file_path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(file_path)?;

        let mut rom_data = Vec::new();
        file.read_to_end(&mut rom_data)?;

        Ok(rom_data)
    }
    
    pub fn read_rom(&self, addr: usize) -> u8 {
        if addr < BOOTROM_SIZE && self.bank == 0 {
            self.bootrom[addr]
        } else {
            self.mbc.read_rom(addr)
        }
    }

    pub fn save_mbc_state(&self) {
        self.mbc.save_state();
    }

    pub fn write_rom(&mut self, addr: usize, byte: u8) {
        self.mbc.write_rom(addr, byte);
    }

    pub fn read_ram(&self, addr: usize) -> u8 {
        self.mbc.read_ram(addr)
    }

    pub fn write_ram(&mut self, addr: usize, byte: u8) {
        self.mbc.write_ram(addr, byte);
    }
}