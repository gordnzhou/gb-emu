use super::{Apu, LengthCounter, MAX_PERIOD};

const WAVE_RAM_SIZE: usize = 32;
const LENGTH_TICKS: u32 = 256;

pub struct Wave {
    nr30: u8,
    nr31: u8,
    nr32: u8,
    nr33: u8,
    nr34: u8,
    wave_ram: [u8 ; WAVE_RAM_SIZE],
    dac_on: bool,
    length_counter: LengthCounter,
    sample_index: usize,
    freq_counter: u32,
}

impl Wave {
    pub fn new() -> Self {
        Wave {
            nr30: 0,
            nr31: 0,
            nr32: 0, 
            nr33: 0,
            nr34: 0,
            wave_ram: [0; WAVE_RAM_SIZE],
            dac_on: false,
            length_counter: LengthCounter::new(LENGTH_TICKS),
            sample_index: 0,
            freq_counter: 0,
        }
    }

    pub fn reset(&mut self) -> Self {
        let wave_ram = self.wave_ram;
        let mut wave = Wave::new();
        wave.wave_ram = wave_ram;
        wave
    }

    pub fn make_sample(&mut self) -> f32 {
        if !self.length_counter.channel_on() || !self.dac_on {
            return 0.0;
        }

        if self.freq_counter >= (MAX_PERIOD - self.period_value()) / 2 {
            self.freq_counter = 0;
            self.sample_index = (self.sample_index + 1) % WAVE_RAM_SIZE;
        }
        self.freq_counter += 1;

        let volume = match (self.nr32 & 0x60) >> 5 {
            0 => 4,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => unreachable!()
        };

        Apu::to_analog(self.wave_ram[self.sample_index] >> volume)
    }

    pub fn step(&mut self, div_apu_tick: bool) {
        self.length_counter.tick(div_apu_tick as u8);
    }

    pub fn read(&self, addr: usize) -> u8 {
        match addr {
            0xFF1A => self.nr30 | 0x7F,
            0xFF1B => self.nr31 | 0xFF,
            0xFF1C => self.nr32 | 0x9F,
            0xFF1D => self.nr33 | 0xFF,
            0xFF1E => self.nr34 | 0xBF,
            0xFF30..=0xFF3F => self.read_wave_ram(addr),
            _ => unreachable!()
        }
    }

    pub fn write(&mut self, addr: usize, byte: u8) {
        match addr {
            0xFF1A => self.write_nr30(byte),
            0xFF1B => self.write_nr31(byte),
            0xFF1C => self.nr32 = byte,
            0xFF1D => self.nr33 = byte,
            0xFF1E => self.write_nr34(byte),
            0xFF30..=0xFF3F => self.write_wave_ram(addr, byte),
            _ => unreachable!()
        };
    }

    pub fn channel_on(&self) -> bool {
        self.length_counter.channel_on() 
    }

    pub fn dac_on(&self) -> bool {
        self.dac_on
    }

    fn read_wave_ram(&self, addr: usize) -> u8 {
        let wave_addr = (addr - 0xFF30) << 1;
        self.wave_ram[wave_addr] << 4 | self.wave_ram[wave_addr + 1]
    }

    fn write_wave_ram(&mut self, addr: usize, byte: u8) {
        let wave_addr = (addr - 0xFF30) << 1;
        self.wave_ram[wave_addr] = (byte & 0xF0) >> 4;
        self.wave_ram[wave_addr + 1] = byte & 0x0F;
    }

    fn write_nr31(&mut self, byte: u8) {
        self.length_counter.set_ticks(byte);
        self.nr31 = byte;
    }

    fn write_nr30(&mut self, byte: u8) {
        self.dac_on = byte & 0x80 != 0;
        self.nr30 = byte;
    }

    fn write_nr34(&mut self, byte: u8) {
        if self.nr34 & 0x80 != 0 {
            self.length_counter.on_trigger();
            self.sample_index = 0;
            self.freq_counter = 0;
        }
        let new_length_enabled = byte & 0x40 != 0;
        self.length_counter.extra_clocking(new_length_enabled);
        self.length_counter.set_tick_enable(new_length_enabled);
        self.nr34 = byte
    }

    fn period_value(&self) -> u32 {
       (self.nr34 as u32 & 7) << 8 | self.nr33 as u32
    }
}