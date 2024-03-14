mod no_mbc;
mod mbc1;

use core::panic;

use self::mbc1::Mbc1;
pub use self::no_mbc::NoMbc;

use super::header::Header;

pub const ROM_MEMORY_SPACE: usize = 0x8000; 
pub const RAM_MEMORY_SPACE: usize = 0x2000; 

pub const ROM_BANK_SIZE: usize = 0x4000;
pub const RAM_BANK_SIZE: usize = 0x2000;

pub trait Mbc {
    fn read_rom(&self, addr: usize) -> u8;
    fn write_rom(&mut self, addr: usize, byte: u8);
    fn read_ram(&self, addr: usize) -> u8;
    fn write_ram(&mut self, addr: usize, byte: u8);
    fn display(&self) -> String;
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
        _ => panic!("Unknown or unimplemented cartridge type!")
    }
}
