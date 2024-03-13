
pub trait Mbc {
    fn new() -> Self;

    fn read_rom(&self, addr: usize) -> u8;

    fn write_rom(&mut self, addr: usize) -> u8;

    fn read_ram(&self, addr: usize) -> u8;

    fn write_ram(&mut self, addr: usize) -> u8;
}
