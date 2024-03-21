use super::{Apu, LengthCounter, MAX_PERIOD, WAVE_RAM_START};

const WAVE_RAM_SIZE: usize = 16;
const LENGTH_TICKS: u32 = 256;

pub struct Wave {
    nr30: u8,
    nr31: u8,
    nr32: u8,
    nr33: u8,
    nr34: u8,
    wave_ram: [u8; WAVE_RAM_SIZE],
    sample_buffer: u8,
    wave_reads_0xff: bool,
    dac_on: bool,
    length_counter: LengthCounter,
    sample_index: usize,
    freq_counter: u32,
    power_on: bool,
}

impl Wave {
    pub fn new() -> Self {
        Wave {
            nr30: 0,
            nr31: 0,
            nr32: 0, 
            nr33: 0,
            nr34: 0xBF,
            wave_ram: [0; WAVE_RAM_SIZE],
            sample_buffer: 0,
            wave_reads_0xff: true,
            dac_on: false,
            length_counter: LengthCounter::new(LENGTH_TICKS),
            sample_index: 0,
            freq_counter: MAX_PERIOD,
            power_on: false,
        }
    }

    pub fn power_off(&mut self) {
        self.power_on = false;
        self.nr30 = 0;
        self.nr31 = 0;
        self.nr32 = 0;
        self.nr33 = 0;
        self.nr34 = 0;
        self.dac_on = false;
        self.length_counter = LengthCounter::new(LENGTH_TICKS);
        self.sample_index = 0;
        self.sample_buffer = 0;
        self.freq_counter = 0;
    }

    pub fn power_on(&mut self) {
        self.power_on = true;
    }

    pub fn make_sample(&mut self) -> f32 {
        if !self.length_counter.channel_on() || !self.dac_on {
            return 0.0;
        }

        let volume = match (self.nr32 & 0x60) >> 5 {
            0 => 4,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => unreachable!()
        };

        if self.freq_counter <= 2 {
            if self.freq_counter == 2 {
                self.wave_reads_0xff = false;
            }

            self.freq_counter += MAX_PERIOD - self.period_value();
            if self.freq_counter > 2 {
                self.freq_counter -= 2;
            }

            self.sample_index = (self.sample_index + 1) % (2 * WAVE_RAM_SIZE);
            self.sample_buffer = self.wave_ram[self.sample_index / 2];
        } else {
            self.freq_counter -= 2;
            self.wave_reads_0xff = true;
        }

        let sample_nibble = if self.sample_index % 2 == 0 {
            (self.sample_buffer & 0xF0) >> 4
        } else {
            self.sample_buffer & 0x0F
        };

        Apu::to_analog(sample_nibble >> volume)
    }

    pub fn frame_sequencer_step(&mut self) {
        self.length_counter.tick();
    }

    pub fn read(&self, addr: usize) -> u8 {
        match addr {
            0xFF1A => self.nr30 | 0x7F,
            0xFF1B => self.nr31 | 0xFF,
            0xFF1C => self.nr32 | 0x9F,
            0xFF1D => self.nr33 | 0xFF,
            0xFF1E => self.nr34 | 0xBF,
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
            _ => unreachable!()
        };
    }

    pub fn channel_on(&self) -> bool {
        self.length_counter.channel_on() 
    }

    pub fn dac_on(&self) -> bool {
        self.dac_on
    }

    pub fn read_wave_ram(&self, addr: usize) -> u8 {
        if self.channel_on() && self.dac_on() {
            // Obscure Behavior (DMG only): reading/writing to wave RAM while 
            // channel is on will access the current sample buffer IF it was updated 
            // by wave RAM at most 2 cycles ago; 
            // otherwise reads return 0xFF and write does nothing.
            if self.wave_reads_0xff {
               0xFF
            } else {
                self.sample_buffer
            }
        } else {
            self.wave_ram[addr - WAVE_RAM_START]
        }
    }

    pub fn write_wave_ram(&mut self, addr: usize, byte: u8) {
        if self.channel_on() && self.dac_on() {
            // Obscure Behavior (DMG only): SEE ABOVE
            if !self.wave_reads_0xff {
                self.wave_ram[self.sample_index / 2] = byte;
            }
        } else {
            self.wave_ram[addr - WAVE_RAM_START] = byte;
        }
    }

    fn write_nr31(&mut self, byte: u8) {
        if self.power_on {
            self.nr31 = byte;
        }
        self.length_counter.set_ticks(byte);
    }

    fn write_nr30(&mut self, byte: u8) {
        self.nr30 = byte;
        self.dac_on = byte & 0xF8 != 0;

        if !self.dac_on {
            self.length_counter.turn_off_channel();
        };
    }

    fn write_nr34(&mut self, byte: u8) {
        self.nr34 = byte;

        // Obscure Behavior (DMG only): Triggering while sample byte is being processed
        // corrupts wave RAM based on the current sample's position.
        if self.channel_on() && self.dac_on && self.freq_counter <= 1 {
            let next_byte_index = ((self.sample_index + 1) / 2) % WAVE_RAM_SIZE;
            if next_byte_index < 4 {
                self.wave_ram[0] = self.wave_ram[next_byte_index];
            } else {
                let segment = (next_byte_index / 4) * 4;
                self.wave_ram.copy_within(segment..=segment+3 , 0);
            }
        }

        let new_length_enabled = byte & 0x40 != 0;
        self.length_counter.extra_clocking(new_length_enabled);
        self.length_counter.set_tick_enable(new_length_enabled);

        if byte & 0x80 != 0 {
            self.length_counter.on_trigger();
            self.freq_counter = MAX_PERIOD - self.period_value() + 3;
            self.sample_buffer = self.wave_ram[self.sample_index / 2];
            self.sample_index = 0;
        }
    }

    fn period_value(&self) -> u32 {
       (self.nr34 as u32 & 7) << 8 | self.nr33 as u32
    }
}