pub struct Apu {

}

impl Apu {
    pub fn new() -> Self {
        Apu { }
    }

    #[allow(dead_code)]
    pub fn step(&mut self, _cycles: u8) {
        // TODO: ...
    }

    pub fn read_io(&self, _addr: usize) -> u8 {
        0xFF // stub
    }

    pub fn read_wave(&self, _addr: usize) -> u8 {
        0xFF // stub
    }

    pub fn write_io(&self, _addr: usize, _byte: u8) {
        // stub
    }

    pub fn write_wave(&self, _addr: usize, _byte: u8) {
        // stub
    }
}