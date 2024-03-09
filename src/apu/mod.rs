mod pulse;
mod wave;
mod noise;

use crate::emulator::{CYCLE_HZ, SAMPLING_RATE_HZ, AUDIO_SAMPLES};
use crate::timer::Stepper;
use self::pulse::{Pulse1, Pulse2};
use self::wave::Wave;
use self::noise::Noise;

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

        let length_steps = self.length_stepper.step(div_apu_tick as u32);
        let _envelope_steps = self.envelope_stepper.step(div_apu_tick as u32);
        let _sweep_steps = self.sweep_stepper.step(div_apu_tick as u32);

        let wave_buffer = self.wave.step(cycles, length_steps);
        
        for sample in wave_buffer {
            if self.sample_gather == (CYCLE_HZ / SAMPLING_RATE_HZ) {
                self.sample_gather = 0;

                let sample = if self.wave.dac_on() { 
                    -1.0 + (sample as f32 / 15.0) * 2.0
                } else { 
                    0.0
                }.clamp(-1.0, 1.0);
                
                let left_sample = sample;
                let right_sample = sample;

                self.audio_buffer[self.buffer_index] = left_sample;
                self.audio_buffer[self.buffer_index] = right_sample;
                self.buffer_index += 2;
            }
            self.sample_gather += 1;
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
        // TODO: should be if their CHANNEL IS ON, not their DAC
        res |=  self.pulse1.dac_on() as u8;
        res |= (self.pulse2.dac_on() as u8) << 1;
        res |= (self.wave.channel_on() as u8) << 2;
        res |= (self.noise.dac_on()  as u8) << 3;
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
        // self.pulse1 = Pulse1::new();
        // self.pulse2 = Pulse2::new();
        self.wave = Wave::new();
        // self.noise = Noise::new();
    }

}