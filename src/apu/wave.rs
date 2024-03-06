use crate::timer::Stepper;

const WAVE_RAM_SIZE: usize = 16;

const MAX_LENGTH: u32 = 256;

pub struct Wave {
    nr30: u8,
    nr31: u8,
    nr32: u8,
    nr33: u8,
    nr34: u8,
    wave_ram: [u8 ; 2 * WAVE_RAM_SIZE],

    silenced: bool,
    length_stepper: Stepper,
    freq_stepper: Stepper,
    sample_index: usize,
}

impl Wave {
    pub fn new() -> Self {
        Wave {
            nr30: 0,
            nr31: 0,
            nr32: 0, 
            nr33: 0,
            nr34: 0,
            wave_ram: [0; 2 * WAVE_RAM_SIZE],

            silenced: true,
            length_stepper: Stepper::new(0, MAX_LENGTH),
            freq_stepper: Stepper::new(0, 2048),
            sample_index: 0,
        }
    }

    /// Returns exactly (2 * cycles) samples.
    pub fn step(&mut self, cycles: u32, length_steps: u32) -> Vec<u8> {
        if self.nr34 & 0x40 != 0 && !self.silenced {
            let length_over = self.length_stepper.step(length_steps);

            if length_over != 0 {
                self.length_stepper.set_period(MAX_LENGTH);
                self.length_stepper.set_steps_so_far(self.nr31 as u32);
                self.silenced = true;
            }
        }

        let mut res = Vec::new();

        let cur_steps = self.freq_stepper.get_steps_so_far();
        self.freq_stepper.step(cycles);
        
        for i in 0..2*cycles {
            if (cur_steps + i) % self.freq_stepper.get_period() == 0 {
                self.sample_index = (self.sample_index + 1) % WAVE_RAM_SIZE;
            }

            let mut sample = if self.silenced {
                0
            } else {
                self.wave_ram[self.sample_index]
            };

            sample >>= self.output_level();

            res.push(sample);
        }

        res
    }

    pub fn dac_on(&self) -> bool {
        self.nr30 & 0x80 != 0
    }

    fn period_value(&self) -> u32 {
        (self.nr34 as u32 & 0x7) << 8 | self.nr33 as u32
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
            0xFF1A => self.nr30 = byte,
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

    fn write_nr31(&mut self, byte: u8) {
        self.nr31 = byte;
    }

    fn write_nr34(&mut self, byte: u8) {
        if self.nr34 & 0x80 != 0 {
            self.sample_index = 0;
            if self.length_stepper.get_steps_so_far() == MAX_LENGTH {
                self.length_stepper.set_steps_so_far(0);
                self.silenced = false;
            }
            self.freq_stepper.set_period((2048 - self.period_value()) >> 1);
        }

        self.nr34 = byte
    }
}