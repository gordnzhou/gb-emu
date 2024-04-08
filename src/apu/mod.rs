mod channels;
mod envelope;
mod length_counter;
mod sweep;

use crate::config::{AUDIO_SAMPLES, SAMPLING_RATE_HZ};
use crate::constants::M_CYCLE_HZ;
use crate::cpu::GBModel;
use self::channels::*;
use envelope::Envelope;
use length_counter::LengthCounter;
use sweep::Sweep;

const MAX_PERIOD: u32 = 2048;

const WAVE_RAM_START: usize = 0xFF30;
const WAVE_RAM_END: usize = 0xFF3F;

pub struct Apu {
    model: GBModel,
    apu_on: bool,
    pulse1: Pulse,
    pulse2: Pulse,
    wave: Wave,
    noise: Noise,
    audio_buffer: [[f32; 2]; AUDIO_SAMPLES * 4],
    buffer_index: usize,
    sample_gather: u32,
    nr52: u8,
    nr51: u8,
    nr50: u8,
    t_cycles: u8,

    pcm12: u8,
    pcm34: u8,
}

impl Apu {
    pub fn new(model: GBModel) -> Self {
        Apu { 
            model,
            apu_on: true,
            pulse1: Pulse::new(true),
            pulse2: Pulse::new(false),
            wave: Wave::new(model),
            noise: Noise::new(),
            audio_buffer: [[0.0; 2]; AUDIO_SAMPLES * 4],
            buffer_index: 0,
            sample_gather: 0,
            nr52: 0,
            nr51: 0,
            nr50: 0,
            t_cycles: 0,

            pcm12: 0,
            pcm34: 0,
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

    pub fn step(&mut self, t_cycles: u32) {
        if !self.apu_on {
            return;
        }

        for _ in 0..t_cycles {
            self.t_cycles = self.t_cycles.wrapping_add(1);
            if self.t_cycles % 4 == 0 {
                
                let pulse1_sample = self.pulse1.make_sample();
                let pulse2_sample = self.pulse2.make_sample();
                let wave_sample = self.wave.make_sample();
                let noise_sample = self.noise.make_sample();

                if matches!(self.model, GBModel::CGB) {
                    self.pcm12 = (pulse2_sample << 4) | pulse1_sample;
                    self.pcm34 = (noise_sample << 4) | wave_sample; 
                }
                
                if self.sample_gather == (M_CYCLE_HZ / SAMPLING_RATE_HZ) {
                    self.sample_gather = 0;
                    self.push_samples_to_buffer(pulse1_sample, pulse2_sample, wave_sample, noise_sample)
                }
                self.sample_gather += 1;
            }
        }
    }

    fn push_samples_to_buffer(&mut self, pulse1_sample: u8, pulse2_sample: u8, wave_sample: u8, noise_sample: u8) {
        if self.buffer_index >= AUDIO_SAMPLES {
            self.buffer_index = 0;
        }

        let pulse1_sample = Apu::to_analog(pulse1_sample);
        let pulse2_sample = Apu::to_analog(pulse2_sample);
        let wave_sample = Apu::to_analog(wave_sample);
        let noise_sample = Apu::to_analog(noise_sample);

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

    pub fn get_audio_output(&mut self) -> Option<[[f32; 2]; AUDIO_SAMPLES]> {
        if self.buffer_index < AUDIO_SAMPLES {
            return None;
        }

        let mut res = [[0.0; 2]; AUDIO_SAMPLES];
        res.copy_from_slice(&self.audio_buffer[0..AUDIO_SAMPLES]);

        for i in AUDIO_SAMPLES..self.buffer_index {
            self.audio_buffer[i - AUDIO_SAMPLES] = self.audio_buffer[i];
        }
        self.buffer_index -= AUDIO_SAMPLES;

        Some(res)
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

            0xFF76 => self.pcm12,
            0xFF77 => self.pcm34,
            _ => 0xFF
        }
    }

    pub fn write_io(&mut self, addr: usize, byte: u8) {
        // NR52 and (for DMG only) all length counters are preserved and writable while APU is powered off
        let apu_off_readable = match self.model {
            GBModel::DMG => vec![0xFF26, 0xFF11, 0xFF16, 0xFF1B, 0xFF20],
            GBModel::CGB => vec![0xFF26],
        };
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

#[cfg(test)]
mod tests {
    use crate::{bus::{RAM_END, RAM_START}, cartridge::Cartridge, cpu::Cpu};

    const DMG_SOUND: &str = "roms/tests/dmg_sound.gb";
    const CGB_SOUND: &str = "roms/tests/cgb_sound.gb";
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
        let mut cpu = Cpu::new(cartridge, crate::cpu::GBModel::DMG);
    
        let mut cycles: u64 = 0;
        let mut test_num = 1;
        while cycles < (1 << 32) {
            cycles += cpu.step() as u64;

            if cycles % (1 << 18) == 0 {
                let mut output = String::new();

                for i in RAM_START..RAM_END {
                    let byte = cpu.read_byte(i as u16);
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

    #[test]
    fn apu_cgb_sound_test() {
        let mut cartridge = Cartridge::from_file(CGB_SOUND, false);
        for i in RAM_START..RAM_END {
            cartridge.write_ram(i, 0);
        }
        let mut cpu = Cpu::new(cartridge, crate::cpu::GBModel::CGB);
    
        let mut cycles: u64 = 0;
        let mut test_num = 1;
        while cycles < (1 << 32) {
            cycles += cpu.step() as u64;

            if cycles % (1 << 18) == 0 {
                let mut output = String::new();

                for i in RAM_START..RAM_END {
                    let byte = cpu.read_byte(i as u16);
                    if byte != 0 {
                        output.push(char::from(byte));
                    }
                }

                if output.contains(TEST_NUMS[test_num - 1]) {
                    let mut pass_output = String::from(TEST_NUMS[test_num - 1]);
                    pass_output.push_str("ok");
                    if output.contains(&pass_output) {
                        println!("cgb_sound: Test #{} Passed", test_num);
                        test_num += 1;
                        if test_num > 12 {
                            break;
                        }
                    } else {
                        panic!("cgb_sound: Test #{} Failed", test_num);
                    }
                }
            }
        } 
    }
}