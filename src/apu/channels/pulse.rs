use super::{Apu, Envelope, LengthCounter, Sweep, MAX_PERIOD};



const LENGTH_TICKS: u32 = 64;

const DUTY_SAMPLE_SIZE: usize = 8;
const DUTY_TABLE: [[u8; DUTY_SAMPLE_SIZE]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

pub struct Pulse {
    nrx0: u8,
    nrx1: u8,
    nrx2: u8,
    nrx3: u8,
    nrx4: u8,
    length_counter: LengthCounter,
    envelope: Envelope,
    sweep: Option<Sweep>,
    dac_on: bool,
    duty_index: usize,
    freq_counter: u32,
}

impl Pulse {
    pub fn new(with_sweep: bool) -> Self {
        let mut sweep = None;
        if with_sweep {
            sweep = Some(Sweep::new());
        }

        Pulse {
            nrx0: 0,
            nrx1: 0,
            nrx2: 0,
            nrx3: 0,
            nrx4: 0,
            length_counter: LengthCounter::new(LENGTH_TICKS),
            envelope: Envelope::new(),
            sweep,
            dac_on: false,
            duty_index: 0,
            freq_counter: 0,
        }
    }

    pub fn make_sample(&mut self) -> f32 {
        if !self.length_counter.channel_on() || !self.dac_on {
            return 0.0;
        }

        if self.freq_counter >= (MAX_PERIOD - self.period_value()) {
            self.freq_counter = 0;
            self.duty_index = (self.duty_index + 1) % DUTY_SAMPLE_SIZE;
        }
        self.freq_counter += 1;

        let duty_select = (self.nrx1 as usize & 0xC0) >> 6;
        let sample = DUTY_TABLE[duty_select][self.duty_index];

        Apu::to_analog(sample * self.envelope.volume())
    }

    pub fn step(&mut self, div_apu_tick: bool) {
        self.length_counter.tick(div_apu_tick as u8);
        self.envelope.step(div_apu_tick as u8);

        match &mut self.sweep {
            Some(sweep) => if !sweep.step(div_apu_tick as u8,&mut self.nrx3, &mut self.nrx4) {
                self.length_counter.turn_off_channel();
            },
            None => {}
        }
    }

    pub fn channel_on(&self) -> bool {
        self.length_counter.channel_on()
    }

    pub fn dac_on(&self) -> bool {
        self.dac_on
    }

    pub fn read(&self, addr: usize) -> u8 {
        match addr {
            0xFF10 => self.nrx0 | 0x80,
            0xFF11 | 0xFF16 => self.nrx1 | 0x3F,
            0xFF12 | 0xFF17 => self.nrx2,
            0xFF13 | 0xFF18 => self.nrx3 | 0xFF,
            0xFF14 | 0xFF19 => self.nrx4 | 0xBF,
            _ => unreachable!()
        }
    }

    pub fn write(&mut self, addr: usize, byte: u8) {
        match addr {
            0xFF10 => self.write_nrx0(byte),
            0xFF11 | 0xFF16 => self.write_nrx1(byte),
            0xFF12 | 0xFF17 => self.write_nrx2(byte),
            0xFF13 | 0xFF18 => self.write_nrx3(byte),
            0xFF14 | 0xFF19 => self.write_nrx4(byte),
            _ => unreachable!()
        }
    }

    fn write_nrx0(&mut self, byte: u8) {
        match &mut self.sweep {
            Some(sweep) => sweep.set(byte),
            None => {}
        }
        self.nrx0 = byte
    }

    fn write_nrx1(&mut self, byte: u8) {
        self.nrx1 = byte;
        self.length_counter.set_ticks(byte);
    }

    fn write_nrx2(&mut self, byte: u8) {
        self.dac_on = byte & 0xF8 != 0;
        self.envelope.set(byte);
        self.nrx2 = byte;
    }

    fn write_nrx3(&mut self, byte: u8) {
        self.nrx3 = byte;
        let new_period = self.period_value();
        match &mut self.sweep {
            Some(sweep) => sweep.set_period(new_period),
            None => {}
        }
    }

    fn write_nrx4(&mut self, byte: u8) {
        let new_length_enabled = byte & 0x40 != 0;
        if self.nrx4 & 0x80 != 0 {
            self.length_counter.on_trigger();
            self.duty_index = 0;
            self.envelope.on_trigger();
            self.freq_counter = 0;

            match &mut self.sweep {
                Some(sweep) => { sweep.on_trigger(); },
                None => {}
            }
        }
        self.length_counter.extra_clocking(new_length_enabled);
        self.length_counter.set_tick_enable(new_length_enabled);

        self.nrx4 = byte;

        let new_period = self.period_value();
        match &mut self.sweep {
            Some(sweep) => sweep.set_period(new_period),
            None => {}
        }
    }

    fn period_value(&self) -> u32 {
        (self.nrx4 as u32 & 7) << 8 | self.nrx3 as u32
    }
}