mod mbc;
mod header;
mod battery;
mod rtc;

use std::fs::File;
use std::io::{self, Read};

use crate::config::{CGB_BOOTROM_PATH, DMG_BOOTROM_PATH};

use self::header::Header;
use self::mbc::Mbc;

const BOOTROM_SIZE: usize = 0x100;

// Represents the 2nd part of the CGB bootrom (right after the header)
const BOOTROM_2_START: usize = 0x200;
const BOOTROM_2_END: usize = 0x900;

pub struct Cartridge {
    bootrom: [u8; BOOTROM_SIZE],
    bootrom2: [u8; BOOTROM_2_END - BOOTROM_2_START],
    cgb_bootrom: bool,
    bank: u8,
    header: Header,
    with_bootrom: bool,
    mbc: Box<dyn Mbc>,
}

impl Cartridge {
    /// Loads cartridge from array slice of bytes (TODO: currently does NOT support bootrom)
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let header = Header::from_bytes(bytes);
        Cartridge { 
            bootrom: [0; BOOTROM_SIZE],
            bootrom2: [0; BOOTROM_2_END - BOOTROM_2_START],
            mbc: mbc::make_mbc(bytes, &header),
            cgb_bootrom: false,
            bank: 1,
            header,
            with_bootrom: false,
        }
    }

    /// Loads cartridge from the given file path (and optionally runs it with boot ROM).
    pub fn from_file(rom_path: &str, with_bootrom: bool) -> Self {
        let header = match Header::from_file(rom_path) {
            Ok(header) => header,
            Err(err) => {
                panic!("Error reading header from {}: {}", rom_path, err);
            }
        };

        let mut bootrom = [0; BOOTROM_SIZE];
        let mut bootrom2 = [0; BOOTROM_2_END - BOOTROM_2_START];
        let mut bank = 1;
        let mut cgb_bootrom = false;
        
        if with_bootrom {
            bank = 0;

            if header.cgb_compatible() {
                cgb_bootrom = true;
                match Cartridge::read_from_file(CGB_BOOTROM_PATH) {
                    Ok(rom_data) => {
                        assert!(rom_data.len() == BOOTROM_SIZE + 0x100 + BOOTROM_2_END - BOOTROM_2_START);
                        for i in 0..BOOTROM_SIZE {
                            bootrom[i] = rom_data[i];
                        }

                        for i in BOOTROM_2_START..BOOTROM_2_END {
                            bootrom2[i - BOOTROM_2_START] = rom_data[i];
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading bootrom file from {}: {}", CGB_BOOTROM_PATH, err);
                    }
                }
            } else {
                match Cartridge::read_from_file(DMG_BOOTROM_PATH) {
                    Ok(rom_data) => {
                        assert!(rom_data.len() == BOOTROM_SIZE);
                        for i in 0..BOOTROM_SIZE {
                            bootrom[i] = rom_data[i];
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading bootrom file from {}: {}", DMG_BOOTROM_PATH, err);
                    }
                }
            }
        }

        let rom_bytes =  match Cartridge::read_from_file(rom_path) {
            Ok(rom) => rom,
            Err(e) => panic!("{}", e),
        };
        let mbc = mbc::make_mbc(&rom_bytes, &header);
        println!("Detected MBC: {}", mbc.display());

        Cartridge { 
            bootrom,
            bootrom2,
            cgb_bootrom,
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

    fn read_from_file(file_path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(file_path)?;

        let mut rom_data = Vec::new();
        file.read_to_end(&mut rom_data)?;

        Ok(rom_data)
    }
    
    pub fn read_rom(&self, addr: usize) -> u8 {
        if self.bank != 0 {
            self.mbc.read_rom(addr)
        } else {
            if addr < BOOTROM_SIZE {
                self.bootrom[addr]
            } else if self.cgb_bootrom && (BOOTROM_2_START..BOOTROM_2_END).contains(&addr) {
                self.bootrom2[addr - BOOTROM_2_START]
            } else {
                self.mbc.read_rom(addr)
            }
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

    #[cfg(target_arch = "wasm32")]
    pub fn load_save(&mut self, data: Vec<u8>, save_type: &str) {
        self.mbc.load_save(data, save_type)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save_id(&self) -> Option<String> {
        self.mbc.save_id()
    }
}