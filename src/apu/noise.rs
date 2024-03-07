

pub struct Noise {
    nr41: u8,
    nr42: u8,
    nr43: u8,
    nr44: u8,
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            nr41: 0,
            nr42: 0,
            nr43: 0, 
            nr44: 0,
        }
    }

    pub fn dac_on(&self) -> bool {
        false
    }

    pub fn read(&self, addr: usize) -> u8 {
        match addr {
            0xFF20 => self.nr41 | 0xFF,
            0xFF21 => self.nr42,
            0xFF22 => self.nr43,
            0xFF23 => self.nr44 | 0xBF,
            _ => unreachable!()
        }
    }

    pub fn write(&mut self, addr: usize, byte: u8) {
        match addr {
            0xFF20 => self.nr41 = byte,
            0xFF21 => self.nr42 = byte,
            0xFF22 => self.nr43 = byte,
            0xFF23 => self.nr44 = byte,
            _ => unreachable!()
        }
    }
}