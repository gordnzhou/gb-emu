mod pulse;
mod wave;
mod noise;

use crate::emulator::{CYCLE_HZ, SAMPLING_RATE_HZ, AUDIO_SAMPLES};
use crate::timer::Stepper;
use self::pulse::{Pulse1, Pulse2};
use self::wave::Wave;
use self::noise::Noise;

const MAX_PERIOD: u32 = 2048;

pub struct Apu {
    pub audio_buffer: [f32; AUDIO_SAMPLES],
    pub buffer_index: usize,
    sample_gather: u32,
    nr52: u8,
    nr51: u8,
    nr50: u8,

    pulse1: Pulse1,
    pulse2: Pulse2,
    wave: Wave,
    noise: Noise,

    apu_on: bool,
    length_stepper: Stepper,
    envelope_stepper: Stepper,
    sweep_stepper: Stepper,
}

impl Apu {
    pub fn new() -> Self {
        Apu { 
            // Audio samples are in the form: [L1, R1, L2, R2, ...] for L and R channels
            audio_buffer: [0.0; AUDIO_SAMPLES],
            buffer_index: 0,
            sample_gather: 0,
            nr52: 0,
            nr51: 0,
            nr50: 0,

            pulse1: Pulse1::new(),
            pulse2: Pulse2::new(),
            wave: Wave::new(),
            noise: Noise::new(),

            apu_on: true,
            length_stepper: Stepper::new(0, 2),
            envelope_stepper: Stepper::new(0, 8),
            sweep_stepper: Stepper::new(0, 4),
        }
    }

    pub fn step(&mut self, cycles: u32, div_apu_tick: bool) {
        if !self.apu_on {
            return;
        }

        let length_step = self.length_stepper.step(div_apu_tick as u32) > 0;
        let envelope_step = self.envelope_stepper.step(div_apu_tick as u32) > 0;
        let sweep_step = self.sweep_stepper.step(div_apu_tick as u32) > 0;

        self.pulse1.step(length_step, envelope_step, sweep_step);
        self.pulse2.step(length_step, envelope_step);
        self.wave  .step(length_step);
        self.noise .step(length_step, envelope_step);
        
        for _ in 0..(4 * cycles) {
            let pulse1_sample = self.pulse1.make_sample();
            let pulse2_sample = self.pulse2.make_sample();
            let wave_sample = self.wave.make_sample();
            let noise_sample = self.noise.make_sample();
            
            if self.sample_gather == (4 * CYCLE_HZ / SAMPLING_RATE_HZ) {
                self.sample_gather = 0;

                let mut right_sample = 0.0;
                if self.nr51 & 0x01 != 0 { right_sample += pulse1_sample }
                if self.nr51 & 0x02 != 0 { right_sample += pulse2_sample }
                if self.nr51 & 0x04 != 0 { right_sample += wave_sample   }
                if self.nr51 & 0x08 != 0 { right_sample += noise_sample  }
                right_sample /= (self.nr51 & 0x0F).count_ones() as f32;
                right_sample *= ((self.nr50 & 7) + 1) as f32 / 8.0;

                let mut left_sample = 0.0;
                if self.nr51 & 0x10 != 0 { left_sample += pulse1_sample }
                if self.nr51 & 0x20 != 0 { left_sample += pulse2_sample }
                if self.nr51 & 0x40 != 0 { left_sample += wave_sample   }
                if self.nr51 & 0x80 != 0 { left_sample += noise_sample  }
                left_sample /= (self.nr51 & 0xF0).count_ones() as f32;
                left_sample *= (((self.nr50 >> 4) & 7) + 1) as f32 / 8.0;

                self.audio_buffer[self.buffer_index] = left_sample;
                self.audio_buffer[self.buffer_index + 1] = right_sample;
                self.buffer_index += 2;
            }
            self.sample_gather += 1;
        }
    }

    pub fn to_analog(sample: u8) -> f32 {
        -1.0 + (sample as f32 / 7.5)
    }

    pub fn read_io(&self, addr: usize) -> u8 {
        match addr {
            0xFF10..=0xFF14 => self.pulse1.read(addr),
            0xFF16..=0xFF19 => self.pulse2.read(addr),
            0xFF1A..=0xFF1E => self.wave.read(addr),
            0xFF20..=0xFF23 => self.noise.read(addr),
            0xFF24 => self.nr50,
            0xFF25 => self.nr51,
            0xFF26 => self.read_nr52(),
            0xFF30..=0xFF3F => self.wave.read(addr),
            _ => 0xFF
        }
    }

    pub fn write_io(&mut self, addr: usize, byte: u8) {
        if !self.apu_on {
            return;
        }

        match addr {
            0xFF10..=0xFF14 => self.pulse1.write(addr, byte),
            0xFF16..=0xFF19 => self.pulse2.write(addr, byte),
            0xFF1A..=0xFF1E => self.wave.write(addr, byte),
            0xFF20..=0xFF23 => self.noise.write(addr, byte),
            0xFF24 => self.nr50 = byte,
            0xFF25 => self.nr51 = byte,
            0xFF26 => self.write_nr52(byte),
            0xFF30..=0xFF3F => self.wave.write(addr, byte),
            _ => {}
        };
    }

    fn read_nr52(&self) -> u8 {
        let mut res = self.nr52 & 0x80;
        res |= (self.pulse1.channel_on() as u8) << 0;
        res |= (self.pulse2.channel_on() as u8) << 1;
        res |= (self.wave  .channel_on() as u8) << 2;
        res |= (self.noise .channel_on() as u8) << 3;
        res | 0x70
    }

    fn write_nr52(&mut self, byte: u8) {
        if (byte & 0x80) ^ (self.nr52 & 0x80) != 0 {
            if byte & 0x80 == 0 {
                self.turn_off_apu();
            } else {
                self.turn_on_apu();
            }
        }

        self.nr52 = (byte & 0x80) | (self.nr52 & 0x7F);
    }

    fn turn_on_apu(&mut self) {
        self.apu_on = true;
    }

    fn turn_off_apu(&mut self) {
        self.apu_on = false;
        self.pulse1 = Pulse1::new();
        self.pulse2 = Pulse2::new();
        self.wave = Wave::new();
        self.noise = Noise::new();
    }

}

struct LengthCounter {
    enabled: bool,
    ticks: u32,
    max_ticks: u32,
}

impl LengthCounter {
    pub fn new(max_ticks: u32) -> Self {
        LengthCounter {
            enabled: false,
            ticks: 0,
            max_ticks,
        }
    }

    /// Ticks LengthCounter, returning false if length has expired.
    pub fn tick(&mut self) -> bool {
        if self.enabled && self.ticks < self.max_ticks {
            self.ticks += 1;
        }

        self.ticks == self.max_ticks
    }

    pub fn on_trigger(&mut self) {
        if self.ticks == self.max_ticks {
            self.ticks = 0;
        }
    }
}

struct Envelope {
    cur_volume: u8,
    sweep_pace: u8, 
    initial_volume: u8,
    envelope_up: bool,
    sweep_ticks: u8,
}

impl Envelope {
    pub fn new() -> Self {
        Envelope {
            cur_volume: 0,
            sweep_ticks: 0,
            sweep_pace: 0,
            initial_volume: 0,
            envelope_up: false,
        }
    }

    pub fn step(&mut self) {
        if self.sweep_pace == 0 {
            return;
        }

        self.sweep_ticks += 1;

        if self.sweep_ticks == self.sweep_pace {
            self.sweep_ticks = 0;

            let next_volume = if self.envelope_up {
                self.cur_volume as i8 + 1
            } else {
                self.cur_volume as i8 - 1
            };

            if 0x0 <= next_volume && next_volume <= 0xF {
                self.cur_volume = next_volume as u8;
            }
        }
    }
    
    pub fn on_trigger(&mut self) {
        self.sweep_ticks = 0;
        self.cur_volume = self.initial_volume;
    }
}

struct Sweep {
    cur_period: u32,
    shadow_period: u32,

    enabled: bool,
    sweep_period: u32,
    sweep_ticks: u32, 

    sweep_up: bool,
    shift: u32,
}

impl Sweep {
    pub fn new() -> Self {
        Sweep {
            cur_period: 0,
            shadow_period: 0,

            enabled: false,
            sweep_period: 8,
            sweep_ticks: 0,

            sweep_up: false,
            shift: 0,    
        }
    }

    /// Returns false if sweep iteration results in channel being disabled
    pub fn step(&mut self, nr13: &mut u8, nr14: &mut u8) -> bool {
        self.sweep_ticks += 1;

        if self.sweep_ticks == self.sweep_period {
            self.sweep_ticks = if self.sweep_period != 0 {
                0
            } else { 8 };

            if self.enabled && self.sweep_period != 0 {
                let next_period = match self.next_period() {
                    Some(next_period) => next_period,
                    _ => return false, 
                };

                if self.shift > 0 {
                    self.shadow_period = next_period;
                    self.cur_period = next_period;
                    *nr13 = (next_period & 0xFF) as u8;
                    *nr14 = (*nr14 & 0xF8) | ((next_period & 0x700) >> 8) as u8;
                    
                    return self.next_period() == None
                }
            }
        } 

        return true;
    }

    /// Returns next period or None if it will result in an overflow.
    fn next_period(&self) -> Option<u32> {
        let next_period = if self.sweep_up {
            self.shadow_period + self.shadow_period >> self.shift
        } else {
            self.shadow_period - self.shadow_period >> self.shift
        };

        if next_period > 0x7FF {
            Some(next_period)
        } else { None }
    }
    
    /// Returns false if sweep iteration results in channel being disabled
    pub fn on_trigger(&mut self) -> bool {
        self.sweep_ticks = if self.sweep_period != 0 {
            0
        } else { 8 };

        self.shadow_period = self.cur_period;
        self.enabled = self.sweep_period > 0 || self.shift > 0;
        
        if self.shift > 0 {
            return self.next_period() == None;
        } else { true }
    }
}