mod pulse;
mod wave;
mod noise;

use crate::timer::Stepper;
use self::pulse::{Pulse1, Pulse2};
use self::wave::Wave;
use self::noise::Noise;

pub struct Apu {
    pub audio_buffer: Vec<i16>,
    nr52: u8,
    nr51: u8,
    nr50: u8,

    pulse1: Pulse1,
    pulse2: Pulse2,
    wave: Wave,
    noise: Noise,

    length_stepper: Stepper,
    envelope_stepper: Stepper,
    sweep_stepper: Stepper,
}

impl Apu {
    pub fn new() -> Self {
        Apu { 
            // Audio samples are in the form: [L1, R1, L2, R2, ...] for L and R channels
            audio_buffer: Vec::new(),
            nr52: 0,
            nr51: 0,
            nr50: 0,

            pulse1: Pulse1::new(),
            pulse2: Pulse2::new(),
            wave: Wave::new(),
            noise: Noise::new(),

            length_stepper: Stepper::new(0, 2),
            envelope_stepper: Stepper::new(0, 8),
            sweep_stepper: Stepper::new(0, 4),
        }
    }

    pub fn step(&mut self, cycles: u32, div_apu_tick: bool) {
        let length_steps = self.length_stepper.step(div_apu_tick as u32);
        let _envelope_steps = self.envelope_stepper.step(div_apu_tick as u32);
        let _sweep_steps = self.sweep_stepper.step(div_apu_tick as u32);

        let wave_buffer = self.wave.step(cycles, length_steps);
        
        // adds (2 * cycles) samples to frame buffer
        for sample in wave_buffer {
            let left_sample = if self.wave.dac_on() { 
                sample as i16 * 1000
            } else { 
                0 
            };

            let right_sample = left_sample;

            self.audio_buffer.push(left_sample);
            self.audio_buffer.push(right_sample);
        }
    }

    pub fn read_io(&self, addr: usize) -> u8 {
        match addr {
            0xFF10..=0xFF14 => self.pulse1.read(addr),
            0xFF16..=0xFF19 => self.pulse2.read(addr),
            0xFF1A..=0xFF1E => self.wave.read(addr),
            0xFF20..=0xFF23 => self.noise.read(addr),
            0xFF24 => self.nr50,
            0xFF25 => self.nr51,
            0xFF26 => self.nr52 | 0x70,
            0xFF30..=0xFF3F => self.wave.read(addr),
            _ => 0xFF
        }
    }

    pub fn write_io(&mut self, addr: usize, byte: u8) {
        match addr {
            0xFF10..=0xFF14 => self.pulse1.write(addr, byte),
            0xFF16..=0xFF19 => self.pulse2.write(addr, byte),
            0xFF1A..=0xFF1E => self.wave.write(addr, byte),
            0xFF20..=0xFF23 => self.noise.write(addr, byte),
            0xFF24 => self.nr50 = byte,
            0xFF25 => self.nr51 = byte,
            0xFF26 => self.nr52 = (byte & 0x80) | (self.nr52 & 0x7F),
            0xFF30..=0xFF3F => self.wave.write(addr, byte),
            _ => {}
        };
    }
}