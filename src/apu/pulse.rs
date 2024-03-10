pub struct Pulse1 {
    nr10: u8,
    nr11: u8,
    nr12: u8,
    nr13: u8,
    nr14: u8,

    dac_on: bool,
    channel_on: bool,
}

impl Pulse1{
    pub fn new() -> Self {
        Pulse1 {
            nr10: 0,
            nr11: 0,
            nr12: 0,
            nr13: 0,
            nr14: 0,

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
            0xFF10 => self.nr10 | 0x80,
            0xFF11 => self.nr11 | 0x3F,
            0xFF12 => self.nr12,
            0xFF13 => self.nr13 | 0xFF,
            0xFF14 => self.nr14 | 0xBF,
            _ => unreachable!()
        }
    }

    pub fn write(&mut self, addr: usize, byte: u8) {
        match addr {
            0xFF10 => self.nr10 = byte,
            0xFF11 => self.nr11 = byte,
            0xFF12 => self.nr12 = byte,
            0xFF13 => self.nr13 = byte,
            0xFF14 => self.nr14 = byte,
            _ => unreachable!()
        }
    }
}

pub struct Pulse2 {
    nr21: u8,
    nr22: u8,
    nr23: u8,
    nr24: u8,

    dac_on: bool,
    channel_on: bool,
}

impl Pulse2 {
    pub fn new() -> Self {
        Pulse2 {
            nr21: 0,
            nr22: 0,
            nr23: 0,
            nr24: 0,

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
            0xFF16 => self.nr21 | 0x3F,
            0xFF17 => self.nr22,
            0xFF18 => self.nr23 | 0xFF,
            0xFF19 => self.nr24 | 0xBF,
            _ => unimplemented!()
        }
    }

    pub fn write(&mut self, addr: usize, byte: u8) {
        match addr {
            0xFF16 => self.nr21 = byte,
            0xFF17 => self.nr22 = byte,
            0xFF18 => self.nr23 = byte,
            0xFF19 => self.nr24 = byte,
            _ => unimplemented!()
        }
    }
}