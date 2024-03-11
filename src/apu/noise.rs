use super::{Envelope, LengthCounter, MAX_PERIOD};

const LENGTH_TICKS: u32 = 64;

pub struct Noise {
    nr41: u8,
    nr42: u8,
    nr43: u8,
    nr44: u8,

    length_counter: LengthCounter,
    dac_on: bool,
    channel_on: bool,
    freq_period: u32,
    envelope: Envelope,
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            nr41: 0,
            nr42: 0,
            nr43: 0, 
            nr44: 0,

            length_counter: LengthCounter::new(LENGTH_TICKS),
            dac_on: false,
            channel_on: false,
            freq_period: MAX_PERIOD,
            envelope: Envelope::new(),
        }
    }

    pub fn make_sample(&mut self) -> f32 {
        if !self.channel_on || !self.dac_on {
            return 0.0;
        }

        0.0
    }

    pub fn step(&mut self, length_step: bool, envelope_step: bool) {
        if !self.channel_on {
            return;
        }

        if length_step {
            if self.length_counter.tick() {
                self.channel_on = false;
            }
        }

        if envelope_step {
            self.envelope.step();
        }

        self.freq_period = self.period_value();
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
            0xFF20 => self.write_nr41(byte),
            0xFF21 => self.nr42 = byte,
            0xFF22 => self.write_nr42(byte),
            0xFF23 => self.write_nr44(byte),
            _ => unreachable!()
        }
    }

    fn write_nr41(&mut self, byte: u8) {
        self.nr41 = byte;
        self.length_counter.set_ticks((byte & 0x3F) as u32);
    }

    fn write_nr42(&mut self, byte: u8) {
        self.dac_on = byte & 0xF8 != 0;
        self.envelope.initial_volume = (byte & 0xF0) >> 4;
        self.envelope.envelope_up = self.nr42 & 8 != 0;
        self.envelope.sweep_pace = byte & 7;

        self.nr42 = byte;
    }

    fn write_nr44(&mut self, byte: u8) {
        if byte & 0x80 != 0 {
            self.channel_on = true;
            self.envelope.on_trigger();
            if self.length_counter.get_ticks() == LENGTH_TICKS {
                self.length_counter.set_ticks(0);
            }
        }

        self.length_counter.set_enabled(byte & 0x40 != 0);
        self.nr44 = byte
    }

    fn period_value(&self) -> u32 {
        (MAX_PERIOD - ((self.nr44 as u32 & 7) << 8 | self.nr43 as u32)) * 2
    }
}