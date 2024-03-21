mod channels;
mod envelope;
mod length_counter;
mod sweep;

use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Duration;

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::{AudioSubsystem, Sdl};

use crate::emulator::CYCLE_HZ;
use self::channels::*;
use envelope::Envelope;
use length_counter::LengthCounter;
use sweep::Sweep;

const MAX_PERIOD: u32 = 2048;

const SAMPLING_RATE_HZ: u32 = 48000;
const AUDIO_SAMPLES: usize = 2048;

const MASTER_VOLUME: f32 = 0.2;

const WAVE_RAM_START: usize = 0xFF30;
const WAVE_RAM_END: usize = 0xFF3F;

pub struct Apu {
    audio_tx: Option<SyncSender<[[f32; 2]; AUDIO_SAMPLES]>>,
    _audio_subsystem: Option<AudioSubsystem>,
    _audio_device: Option<AudioDevice<Callback>>,
    audio_buffer: [[f32; 2]; AUDIO_SAMPLES * 4],
    buffer_index: usize,
    sample_gather: u32,
    nr52: u8,
    nr51: u8,
    nr50: u8,

    pulse1: Pulse,
    pulse2: Pulse,
    wave: Wave,
    noise: Noise,

    apu_on: bool,
}

impl Apu {
    pub fn new(sdl: Option<Sdl>) -> Self {
        let mut audio_tx = None;
        let mut _audio_subsystem = None;
        let mut _audio_device = None;

        match sdl {
            Some(sdl_context) => {
                let (_audio_tx, audio_rx) = std::sync::mpsc::sync_channel(4);
                audio_tx = Some(_audio_tx);

                let desired_spec = AudioSpecDesired {
                    freq: Some(SAMPLING_RATE_HZ as i32),
                    channels: Some(2),
                    samples: Some(AUDIO_SAMPLES as u16),
                };

                let audio_subsystem = sdl_context.audio().unwrap();
                let audio_device = audio_subsystem.open_playback(None, &desired_spec, |_spec| {
                    Callback { audio_rx, prev_sample: [0.0; 2] }
                }).unwrap();
                audio_device.resume();

                _audio_device = Some(audio_device);
                _audio_subsystem = Some(audio_subsystem);
            }
            None => {}
        }

        Apu { 
            audio_tx,
            _audio_device,
            _audio_subsystem,
            audio_buffer: [[0.0; 2]; AUDIO_SAMPLES * 4],
            buffer_index: 0,
            sample_gather: 0,
            nr52: 0,
            nr51: 0,
            nr50: 0,

            pulse1: Pulse::new(true),
            pulse2: Pulse::new(false),
            wave: Wave::new(),
            noise: Noise::new(),

            apu_on: true,
        }
    }

    pub fn frame_sequencer_step(&mut self) {
        if !self.apu_on {
            return;
        }

        self.pulse1.frame_sequencer_step();
        self.pulse2.frame_sequencer_step();
        self.wave  .frame_sequencer_step();
        self.noise .frame_sequencer_step();
    }

    pub fn step(&mut self, cycles: u32) {
        if !self.apu_on {
            return;
        }
        
        for _ in 0..cycles {
            let pulse1_sample = self.pulse1.make_sample();
            let pulse2_sample = self.pulse2.make_sample();
            let wave_sample = self.wave.make_sample();
            let noise_sample = self.noise.make_sample();
            
            if self.sample_gather == (CYCLE_HZ / SAMPLING_RATE_HZ) {
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

                self.audio_buffer[self.buffer_index][0] = left_sample;
                self.audio_buffer[self.buffer_index][1] = right_sample;
                self.buffer_index += 1;
            }
            self.sample_gather += 1;
        }

        self.push_buffer_samples();
    }

    pub fn push_buffer_samples(&mut self) {
        if self.buffer_index < AUDIO_SAMPLES {
            return;
        }

        match &mut self.audio_tx {
            Some(audio_tx) => {
                let mut res = [[0.0; 2]; AUDIO_SAMPLES];
                res.copy_from_slice(&self.audio_buffer[0..AUDIO_SAMPLES]);
                for i in AUDIO_SAMPLES..self.buffer_index {
                    self.audio_buffer[i - AUDIO_SAMPLES] = self.audio_buffer[i];
                }
                self.buffer_index -= AUDIO_SAMPLES;

                audio_tx.send(res).unwrap();
            }
            None => {
                self.buffer_index = 0;
            }
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
            WAVE_RAM_START..=WAVE_RAM_END => self.wave.read_wave_ram(addr),
            _ => 0xFF
        }
    }

    pub fn write_io(&mut self, addr: usize, byte: u8) {
        // (for DMG only) NR52 and all length counters are preserved and writable while APU is powered off
        let apu_off_readable = vec![0xFF26, 0xFF11, 0xFF16, 0xFF1B, 0xFF20];
        if !self.apu_on && !apu_off_readable.contains(&addr) {
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
            WAVE_RAM_START..=WAVE_RAM_END => self.wave.write_wave_ram(addr, byte),
            _ => {}
        };
    }

    fn read_nr52(&self) -> u8 {
        let mut res = self.nr52 & 0x80;
        res |= ((self.pulse1.channel_on() && self.pulse1.dac_on()) as u8) << 0;
        res |= ((self.pulse2.channel_on() && self.pulse2.dac_on())as u8) << 1;
        res |= ((self.wave  .channel_on() && self.wave  .dac_on()) as u8) << 2;
        res |= ((self.noise .channel_on() && self.noise .dac_on()) as u8) << 3;
        res | 0x70
    }

    fn write_nr52(&mut self, byte: u8) {
        if (byte ^ self.nr52) & 0x80 != 0 {
            if byte & 0x80 == 0 {
                self.turn_off_apu();
            } else {
                self.turn_on_apu();
            }
        }

        self.nr52 = byte & 0x80;
    }

    fn turn_on_apu(&mut self) {
        self.apu_on = true;
        self.pulse1.power_on();
        self.pulse2.power_on();
        self.wave.power_on();
        self.noise.power_on();
    }

    fn turn_off_apu(&mut self) {
        self.apu_on = false;
        self.nr50 = 0;
        self.nr51 = 0;
        self.pulse1.power_off();
        self.pulse2.power_off();
        self.wave.power_off();
        self.noise.power_off();
    }

}

struct Callback {
    audio_rx: Receiver<[[f32; 2]; AUDIO_SAMPLES]>,
    prev_sample: [f32; 2],
}

impl AudioCallback for Callback {
    type Channel = f32;

    fn callback(&mut self, stream: &mut [f32]) {
        match self.audio_rx.recv_timeout(Duration::from_millis(30)) {
            Ok(buffer) => {
                for i in 0..buffer.len() {
                    stream[i * 2] = buffer[i][0];
                    stream[i * 2 + 1] = buffer[i][1];
                }
            
                self.prev_sample = buffer[buffer.len() - 1];
            }
            Err(_) => {
                for i in 0..stream.len() {
                    stream[i] = self.prev_sample[i % 2];
                }
            }
        }

        for i in 0..stream.len() {
            stream[i] *= MASTER_VOLUME
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bus::{RAM_END, RAM_START}, cartridge::Cartridge, cpu::Cpu};

    const DMG_SOUND: &str = "roms/tests/dmg_sound.gb";
    const TEST_NUMS: [&str; 12] = [
        "01:", "02:", "03:", "04:", 
        "05:", "06:", "07:", "08:", 
        "09:", "10:", "11:", "12:", 
    ];

    #[test]
    fn apu_dmg_sound_test() {
        let mut cartridge = Cartridge::from_file(DMG_SOUND, false);
        for i in RAM_START..RAM_END {
            cartridge.write_ram(i, 0);
        }
        let mut cpu = Cpu::new(cartridge);
    
        let mut cycles: u64 = 0;
        let mut test_num = 1;
        while cycles < (1 << 32) {
            cycles += cpu.step() as u64;

            if cycles % (1 << 18) == 0 {
                let mut output = String::new();

                for i in RAM_START..RAM_END {
                    let byte = cpu.bus.read_byte(i as u16);
                    if byte != 0 {
                        output.push(char::from(byte));
                    }
                }

                if output.contains(TEST_NUMS[test_num - 1]) {
                    let mut pass_output = String::from(TEST_NUMS[test_num - 1]);
                    pass_output.push_str("ok");
                    if output.contains(&pass_output) {
                        println!("dmg_sound: Test #{} Passed", test_num);
                        test_num += 1;
                        if test_num > 12 {
                            break;
                        }
                    } else {
                        panic!("dmg_sound: Test #{} Failed", test_num);
                    }
                }
            }
        } 
    }
}