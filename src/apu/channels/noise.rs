use super::{Apu, Envelope, LengthCounter};

const LENGTH_TICKS: u32 = 64;

pub struct Noise {
    nr41: u8,
    nr42: u8,
    nr43: u8,
    nr44: u8,

    length_counter: LengthCounter,
    lfsr: Lfsr,
    envelope: Envelope,
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

            length_counter: LengthCounter::new(LENGTH_TICKS),
            lfsr: Lfsr::new(),
            envelope: Envelope::new(),
            dac_on: false,
            channel_on: false,
        }
    }

    pub fn make_sample(&mut self) -> f32 {
        if !self.channel_on || !self.dac_on {
            return 0.0;
        }

        let sample = self.lfsr.next_sample();

        Apu::to_analog(sample * self.envelope.cur_volume)
    }

    pub fn step(&mut self, length_step: bool, envelope_step: bool) {
        if !self.channel_on {
            return;
        }

        if envelope_step {
            self.envelope.step();
        }

        if length_step {
            if self.length_counter.tick() {
                self.channel_on = false;
            }
        }
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
            0xFF21 => self.write_nr42(byte),
            0xFF22 => self.write_nr43(byte),
            0xFF23 => self.write_nr44(byte),
            _ => unreachable!()
        }
    }

    fn write_nr41(&mut self, byte: u8) {
        self.nr41 = byte;
        self.length_counter.ticks = (byte & 0x3F) as u32;
    }

    fn write_nr42(&mut self, byte: u8) {
        if byte & 0xF8 == 0 {
            self.dac_on = false;
            self.channel_on = false;
        } else {
            self.dac_on = true;
        }

        self.envelope.initial_volume = (byte & 0xF0) >> 4;
        self.envelope.envelope_up = (byte & 8) != 0;
        self.envelope.sweep_pace = byte & 7;

        self.nr42 = byte;
    }

    fn write_nr43(&mut self, byte: u8) {
        self.lfsr.divisor_code = byte & 7;
        self.lfsr.width = byte & 8 != 0;
        self.lfsr.shift_amount = (byte & 0xF0) >> 4;
        self.nr43 = byte;
    }

    fn write_nr44(&mut self, byte: u8) {
        if byte & 0x80 != 0 {
            self.channel_on = true;
            self.envelope.on_trigger();
            self.lfsr.shift_register = 0xFFFF;
            self.length_counter.on_trigger()
        }

        self.length_counter.enabled = byte & 0x40 != 0;
        self.nr44 = byte
    }
}

struct Lfsr {
    shift_register: u16,
    width: bool,
    shift_period: u32,
    divisor_code: u8,
    shift_amount: u8,
}

impl Lfsr {
    pub fn new() -> Self {
        Lfsr {
            shift_register: 0xFFFF,
            width: false,
            shift_period: 0,
            divisor_code: 0,
            shift_amount: 0,
        }
    }

    pub fn next_sample(&mut self) -> u8 {
        if self.shift_period == 0 {
            self.set_shift_period();
            self.do_shift()
        } else {
            self.shift_period -= 1;
        }

        (!self.shift_register as u8) & 0x01
    }

    fn do_shift(&mut self) {
        let xor_bit = (self.shift_register & 0b01) ^ ((self.shift_register & 0b10) >> 1);
        self.shift_register = (self.shift_register >> 1) | (xor_bit << 14);

        if self.width {
            self.shift_register &= !(1 << 6);
            self.shift_register |= xor_bit << 6;
        }
    }

    fn set_shift_period(&mut self) {
        let divisor = match self.divisor_code {
            0 => 8,
            1 => 16,
            2 => 32,
            3 => 48,
            4 => 64,
            5 => 80,
            6 => 96,
            7 => 112,
            _ => unreachable!()
        };

        self.shift_period = divisor << self.shift_amount;
    }
}