mod no_mbc;
mod mbc1;
mod mbc3;
mod mbc2;
mod mbc5;

use core::panic;
use std::fs::File;
use std::io::{self, Read};

use self::mbc1::Mbc1;
use self::mbc2::Mbc2;
use self::mbc3::Mbc3;
use self::mbc5::Mbc5;
use self::no_mbc::NoMbc;

use super::header::Header;

pub const ROM_MEMORY_SPACE: usize = 0x8000; 
pub const RAM_MEMORY_SPACE: usize = 0x2000; 

pub const ROM_BANK_SIZE: usize = 0x4000;
pub const RAM_BANK_SIZE: usize = 0x2000;

pub trait Mbc {
    /// Handles bus reads from 0x0000 to 0x7FFF
    fn read_rom(&self, addr: usize) -> u8;

    /// Handles bus writes from 0x0000 to 0x7FFF
    fn write_rom(&mut self, addr: usize, byte: u8);
    
    /// Handles bus reads from 0xA000 to 0xBFFF
    fn read_ram(&self, addr: usize) -> u8;

    /// Handles bus reads from 0xA000 to 0xBFFF
    fn write_ram(&mut self, addr: usize, byte: u8);

    /// Displays Mbc specifications.
    fn display(&self) -> String;

    /// Handles saving of MBC state (if it includes battery).
    fn save_state(&self);
}

pub fn make_mbc(rom_path: &str, header: &Header) -> Box<dyn Mbc> {
    let rom_banks = header.num_rom_banks();
    let ram_banks = header.num_ram_banks();
    let title = header.title();

    match header.cartridge_type() {
        0x00 => Box::new(NoMbc::new(rom_path)),
        0x01 => Box::new(Mbc1::new(rom_path, rom_banks)),
        0x02 => Box::new(Mbc1::new(rom_path, rom_banks).with_ram(ram_banks)),
        0x03 => Box::new(Mbc1::new(rom_path, rom_banks).with_ram(ram_banks).with_battery(title)),
        0x05 => Box::new(Mbc2::new_with_ram(rom_path, rom_banks)),
        0x06 => Box::new(Mbc2::new_with_ram(rom_path, rom_banks).with_battery(title)),
        0x08 => unimplemented!(),
        0x09 => unimplemented!(),
        0x0B => unimplemented!(),
        0x0C => unimplemented!(),
        0x0D => unimplemented!(),
        0x0F => Box::new(Mbc3::new(rom_path, rom_banks).with_rtctimer().with_battery(title)),
        0x10 => Box::new(Mbc3::new(rom_path, rom_banks).with_rtctimer().with_ram(ram_banks).with_battery(title)),
        0x11 => Box::new(Mbc3::new(rom_path, rom_banks)),
        0x12 => Box::new(Mbc3::new(rom_path, rom_banks).with_ram(ram_banks)),
        0x13 => Box::new(Mbc3::new(rom_path, rom_banks).with_ram(ram_banks).with_battery(title)),
        0x19 => Box::new(Mbc5::new(rom_path, rom_banks)),
        0x1A => Box::new(Mbc5::new(rom_path, rom_banks).with_ram(ram_banks)),
        0x1B => Box::new(Mbc5::new(rom_path, rom_banks).with_ram(ram_banks).with_battery(title)),
        0x1C => Box::new(Mbc5::new(rom_path, rom_banks).with_rumble()),
        0x1D => Box::new(Mbc5::new(rom_path, rom_banks).with_rumble().with_ram(ram_banks)),
        0x1E => Box::new(Mbc5::new(rom_path, rom_banks).with_rumble().with_ram(ram_banks).with_battery(title)),
        0x20 => unimplemented!(),
        0x22 => unimplemented!(),
        0xFC => unimplemented!(),
        0xFD => unimplemented!(),
        0xFE => unimplemented!(),
        0xFF => unimplemented!(),
        _ => panic!("Unknown cartridge type!")
    }
}

fn read_rom_from_file(rom_path: &str, rom_banks: usize) -> io::Result<Vec<[u8; ROM_BANK_SIZE]>> {
    let mut file = File::open(rom_path)?;

    let mut rom_data = Vec::new();
    file.read_to_end(&mut rom_data)?;

    let mut rom = vec![[0; ROM_BANK_SIZE]; rom_banks];
    for i in 0..rom_data.len() {
        rom[i / ROM_BANK_SIZE][i % ROM_BANK_SIZE] = rom_data[i];
    }

    Ok(rom)
}
