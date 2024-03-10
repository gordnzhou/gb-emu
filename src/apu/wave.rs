use crate::timer::Stepper;

const WAVE_RAM_SIZE: usize = 32;

const MAX_LENGTH: u32 = 256;
const MAX_PERIOD: u32 = 2048;

pub struct Wave {
    nr30: u8,
    nr31: u8,
    nr32: u8,
    nr33: u8,
    nr34: u8,
    wave_ram: [u8 ; WAVE_RAM_SIZE],

    dac_on: bool,
    channel_on: bool,
    length_stepper: Stepper,
    sample_index: usize,
    freq_counter: u32,
    freq_period: u32,
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
            channel_on: false,
            length_stepper: Stepper::new(0, MAX_LENGTH),
            sample_index: 0,
            freq_counter: 0,
            freq_period: MAX_PERIOD,
        }
    }

    pub fn step(&mut self, cycles: u32, length_steps: u32) -> Vec<u8> {
        if self.nr34 & 0x40 != 0 && length_steps > 0 {
            let length_over = self.length_stepper.step(length_steps);

            if length_over != 0 {
                self.length_stepper.set_steps_so_far(256);
                self.channel_on = false;
            }
        } 

        let mut res = Vec::new();

        self.freq_period = self.period_value();
        for _ in 0..(4 * cycles) {
            if !self.channel_on {
                res.push(0);
                continue;
            }

            if self.freq_counter >= self.freq_period {
                self.freq_counter = 0;
                self.sample_index = (self.sample_index + 1) % WAVE_RAM_SIZE;
            }
            self.freq_counter += 1;

            res.push(self.wave_ram[self.sample_index] >> self.output_level());
        }

        res
    }

    fn output_level(&self) -> u8 {
        match (self.nr32 & 0x60) >> 5 {
            0 => 4,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => unreachable!()
        }
    }

    pub fn read(&self, addr: usize) -> u8 {
        match addr {
            0xFF1A => self.nr30 | 0x7F,
            0xFF1B => self.nr31 | 0xFF,
            0xFF1C => self.nr32 | 0x9F,
            0xFF1D => self.nr33 | 0xFF,
            0xFF1E => self.nr34 | 0xBF,
            0xFF30..=0xFF3F => {
                let wave_addr = (addr - 0xFF30) << 1;
                self.wave_ram[wave_addr] << 4 | self.wave_ram[wave_addr + 1]
            }
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
            0xFF30..=0xFF3F => {
                let wave_addr = (addr - 0xFF30) << 1;
                self.wave_ram[wave_addr] = (byte & 0xF0) >> 4;
                self.wave_ram[wave_addr + 1] = byte & 0x0F;
            }
            _ => unreachable!()
        };
    }

    pub fn dac_on(&self) -> bool {
        self.dac_on
    }

    pub fn channel_on(&self) -> bool {
        self.channel_on
    }

    fn write_nr31(&mut self, byte: u8) {
        self.nr31 = byte;
        self.length_stepper.set_steps_so_far(byte as u32);
    }

    fn write_nr30(&mut self, byte: u8) {
        self.nr30 = byte;

        if self.nr30 & 0x80 == 0 {
            self.dac_on = false;
            self.channel_on = false;
        } else {
            self.dac_on = true;
        }
    }

    fn write_nr34(&mut self, byte: u8) {
        if byte & 0x80 != 0 {
            self.channel_on = true;
            // println!("trigger {}", self.period_value());
            // if self.length_stepper.get_steps_so_far() == MAX_LENGTH {
            //     self.length_stepper.set_steps_so_far(0);
            // }
        }

        self.nr34 = byte
    }

    fn period_value(&self) -> u32 {
        (2048 - ((self.nr34 as u32 & 7) << 8 | self.nr33 as u32)) * 2
    }
}