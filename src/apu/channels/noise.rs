use super::{Envelope, LengthCounter};

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
    power_on: bool,
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            nr41: 0,
            nr42: 0,
            nr43: 0, 
            nr44: 0xBF,
            length_counter: LengthCounter::new(LENGTH_TICKS),
            lfsr: Lfsr::new(),
            envelope: Envelope::new(),
            dac_on: false,
            power_on: false,
        }
    }

    pub fn power_off(&mut self) {
        self.power_on = false;
        self.nr41 = 0;
        self.nr42 = 0;
        self.nr43 = 0;
        self.nr44 = 0;
        self.length_counter = LengthCounter::new(LENGTH_TICKS);
        self.lfsr = Lfsr::new();
        self.envelope = Envelope::new();
        self.dac_on = false;
    }

    pub fn power_on(&mut self) {
        self.power_on = true;
    }

    pub fn make_sample(&mut self) -> u8 {
        if !self.length_counter.channel_on() || !self.dac_on {
            return 0;
        }
        let sample = self.lfsr.next_sample();
        sample * self.envelope.volume()
    }

    pub fn frame_sequencer_step(&mut self) {
        self.length_counter.tick();
        self.envelope.step();
    }

    pub fn channel_on(&self) -> bool {
        self.length_counter.channel_on()
    }

    pub fn dac_on(&self) -> bool {
        self.dac_on
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
        if self.power_on {
            self.nr41 = byte;
        }
        self.length_counter.set_ticks(byte)
    }

    fn write_nr42(&mut self, byte: u8) {
        self.nr42 = byte;

        self.envelope.set(byte);
        self.dac_on = byte & 0xF8 != 0;

        if !self.dac_on {
            self.length_counter.turn_off_channel();
        };
    }

    fn write_nr43(&mut self, byte: u8) {
        self.nr43 = byte;
        self.lfsr.divisor_code = byte & 7;
        self.lfsr.width = byte & 8 != 0;
        self.lfsr.shift_amount = (byte & 0xF0) >> 4;
    }

    fn write_nr44(&mut self, byte: u8) {
        self.nr44 = byte;

        let new_length_enabled = byte & 0x40 != 0;
        self.length_counter.extra_clocking(new_length_enabled);
        self.length_counter.set_tick_enable(new_length_enabled);

        if byte & 0x80 != 0 {
            self.length_counter.on_trigger();
            self.envelope.on_trigger();
            self.lfsr.shift_register = 0xFFFF;
        }
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