use super::{Apu, Envelope, LengthCounter, Sweep, MAX_PERIOD};



const LENGTH_TICKS: u32 = 64;

const DUTY_SAMPLE_SIZE: usize = 8;
const DUTY_TABLE: [[u8; DUTY_SAMPLE_SIZE]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

pub struct Pulse1 {
    nr10: u8,
    nr11: u8,
    nr12: u8,
    nr13: u8,
    nr14: u8,

    length_counter: LengthCounter,
    envelope: Envelope,
    sweep: Sweep,
    dac_on: bool,
    channel_on: bool,
    duty_index: usize,
    freq_counter: u32,
}

impl Pulse1{
    pub fn new() -> Self {
        Pulse1 {
            nr10: 0,
            nr11: 0,
            nr12: 0,
            nr13: 0,
            nr14: 0,

            length_counter: LengthCounter::new(LENGTH_TICKS),
            envelope: Envelope::new(),
            sweep: Sweep::new(),
            dac_on: false,
            channel_on: false,
            duty_index: 0,
            freq_counter: 0,
        }
    }

    pub fn make_sample(&mut self) -> f32 {
        if !self.channel_on || !self.dac_on {
            return 0.0;
        }

        if self.freq_counter >= 2048 - self.period_value() {
            self.freq_counter = 0;
            self.duty_index = (self.duty_index + 1) % DUTY_SAMPLE_SIZE;
        }
        self.freq_counter += 1;

        let duty_select = (self.nr11 as usize & 0xC0) >> 6;
        let sample = DUTY_TABLE[duty_select][self.duty_index];

        Apu::to_analog(sample * self.envelope.cur_volume)
    }

    pub fn step(&mut self, div_apu_tick: bool) {
        if self.length_counter.tick(div_apu_tick as u8) {
            self.channel_on = false;
        }
        self.envelope.step(div_apu_tick as u8);
        self.channel_on &= self.sweep.step(div_apu_tick as u8,&mut self.nr13, &mut self.nr14);
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
            0xFF10 => self.write_nr10(byte),
            0xFF11 => self.write_nr11(byte),
            0xFF12 => self.write_nr12(byte),
            0xFF13 => self.write_nr13(byte),
            0xFF14 => self.write_nr14(byte),
            _ => unreachable!()
        }
    }

    fn write_nr10(&mut self, byte: u8) {
        self.sweep.shift = byte as u32 & 0x07;
        self.sweep.sweep_up = byte & 0x08 == 0;
        self.sweep.sweep_period = (byte as u32 & 0x70) >> 4;
        self.nr10 = byte
    }

    fn write_nr11(&mut self, byte: u8) {
        self.nr11 = byte;
        self.length_counter.set_ticks((byte & 0x3F) as u32);
    }

    fn write_nr12(&mut self, byte: u8) {
        if byte & 0xF8 == 0 {
            self.dac_on = false;
            self.channel_on = false;
        } else {
            self.dac_on = true;
        }

        self.envelope.initial_volume = (byte & 0xF0) >> 4;
        self.envelope.envelope_up = byte & 8 != 0;
        self.envelope.sweep_pace = byte & 7;

        self.nr12 = byte;
    }

    fn write_nr13(&mut self, byte: u8) {
        self.nr13 = byte;
        self.sweep.cur_freq_period = self.period_value();
    }

    fn write_nr14(&mut self, byte: u8) {
        let new_length_enabled = byte & 0x40 != 0;
        if self.nr14 & 0x80 != 0 {
            if self.dac_on {
                self.channel_on = true;
                self.length_counter.on_trigger();
            }
            self.duty_index = 0;
            self.envelope.on_trigger();
            self.sweep.on_trigger();
            self.freq_counter = 0;
        }
        self.channel_on &= self.length_counter.extra_clocking(new_length_enabled);
        self.length_counter.enabled = new_length_enabled;
        self.nr14 = byte;
        self.sweep.cur_freq_period = self.period_value();
    }

    fn period_value(&self) -> u32 {
        (self.nr14 as u32 & 7) << 8 | self.nr13 as u32
    }
}

pub struct Pulse2 {
    nr21: u8,
    nr22: u8,
    nr23: u8,
    nr24: u8,

    length_counter: LengthCounter,
    envelope: Envelope,
    dac_on: bool,
    channel_on: bool,
    duty_index: usize,
    freq_period: u32,
    freq_counter: u32,
}

impl Pulse2 {
    pub fn new() -> Self {
        Pulse2 {
            nr21: 0,
            nr22: 0,
            nr23: 0,
            nr24: 0,

            length_counter: LengthCounter::new(LENGTH_TICKS),
            envelope: Envelope::new(),
            dac_on: false,
            channel_on: false,
            duty_index: 0,
            freq_period: MAX_PERIOD,
            freq_counter: 0,
        }
    }

    pub fn make_sample(&mut self) -> f32 {
        if !self.channel_on || !self.dac_on {
            return 0.0;
        }

        if self.freq_counter >= self.freq_period {
            self.freq_counter = 0;
            self.duty_index = (self.duty_index + 1) % DUTY_SAMPLE_SIZE;
        }
        self.freq_counter += 1;

        let duty_select = (self.nr21 as usize & 0xC0) >> 6;
        let sample = DUTY_TABLE[duty_select][self.duty_index];

        Apu::to_analog(sample * self.envelope.cur_volume)
    }

    pub fn step(&mut self, div_apu_tick: bool) {
        if self.length_counter.tick(div_apu_tick as u8) {
            self.channel_on = false;
        }
        
        self.envelope.step(div_apu_tick as u8);
        self.freq_period = self.period_value();
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
            _ => unreachable!()
        }
    }

    pub fn write(&mut self, addr: usize, byte: u8) {
        match addr {
            0xFF16 => self.write_nr21(byte),
            0xFF17 => self.write_nr22(byte),
            0xFF18 => self.nr23 = byte,
            0xFF19 => self.write_nr24(byte),
            _ => unreachable!()
        }
    }

    fn write_nr21(&mut self, byte: u8) {
        self.nr21 = byte;
        self.length_counter.set_ticks((byte & 0x3F) as u32);
    }

    fn write_nr22(&mut self, byte: u8) {
        if byte & 0xF8 == 0 {
            self.dac_on = false;
            self.channel_on = false;
        } else {
            self.dac_on = true;
        }

        self.envelope.initial_volume = (byte & 0xF0) >> 4;
        self.envelope.envelope_up = byte & 8 != 0;
        self.envelope.sweep_pace = byte & 7;

        self.nr22 = byte;
    }

    fn write_nr24(&mut self, byte: u8) {
        let new_length_enabled = byte & 0x40 != 0;
        if self.nr24 & 0x80 != 0 {
            if self.dac_on {
                self.channel_on = true;
                self.length_counter.on_trigger();
            }
            self.duty_index = 0;
            self.envelope.on_trigger();
            self.freq_counter = 0;
        }
        self.channel_on &= self.length_counter.extra_clocking(new_length_enabled);
        self.length_counter.enabled = new_length_enabled;
        self.nr24 = byte;
    }

    fn period_value(&self) -> u32 {
        MAX_PERIOD - ((self.nr24 as u32 & 7) << 8 | self.nr23 as u32)
    }
}