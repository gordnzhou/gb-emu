

pub struct Noise {
    nr41: u8,
    nr42: u8,
    nr43: u8,
    nr44: u8,

    dac_on: bool,
    channel_on: bool,
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            nr41: 0,
            nr42: 0,
            nr43: 0, 
            nr44: 0,

            dac_on: false,
            channel_on: false,
        }
    }

    pub fn make_sample(&mut self) -> f32 {
        if !self.channel_on || !self.dac_on {
            return 0.0;
        }
        
        0.0
    }

    pub fn step(&mut self, _length_steps: u32) {
        // stub
    }

    pub fn channel_on(&self) -> bool {
        self.channel_on
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