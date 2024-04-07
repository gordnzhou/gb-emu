// div is incremented every 256 T-cycles / 64 M-cycles 
const T_CYCLES_PER_DIV_INC: u32 = 256;

pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,

    div_stepper: Stepper,
    tima_stepper: Stepper,

    next_tma: i32,
}

impl Timer {
    pub fn new() -> Self {
        Timer { 
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,

            div_stepper: Stepper::new(0, T_CYCLES_PER_DIV_INC),
            tima_stepper: Stepper::new(0, 1024),

            next_tma: -1,
        }
    }

    /// Ticks timer registers over the given period (in t cycles); returns true if TIMA overflowed
    pub fn step(&mut self, t_cycles: u32) -> bool {
        let t_cycles = t_cycles;

        let steps = self.div_stepper.step(t_cycles);

        // DIV increments at most once per CPU instruction
        if steps > 0 {
            self.div = self.div.wrapping_add(1);
        }

        if self.tac & 0x04 != 0 {
            self.step_tima(t_cycles)
        } else {
            false
        }
    }

    fn step_tima(&mut self, t_cycles: u32) -> bool {
        self.tima_stepper.set_period(match self.tac & 0x03 {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => unreachable!(),
        });

        let mut tima_overflow = false;

        let steps = self.tima_stepper.step(t_cycles);
        for _ in 0..steps {
            self.tima = self.tima.wrapping_add(1);

            if self.tima == 0 {
                tima_overflow = true;
                self.tima = self.tma;
            }
        }

        if self.next_tma != -1 {
            self.tma = self.next_tma as u8;
            self.next_tma = -1;
        }

        tima_overflow
    }

    pub fn read_io(&self, addr: usize) -> u8 {
        match addr {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => unreachable!()
        }
    }

    pub fn write_io(&mut self, addr: usize, byte: u8) {
        match addr {
            0xFF04 => self.div = 0x00,
            0xFF05 => self.tima = byte,
            0xFF06 => self.next_tma = byte as i32,
            0xFF07 => self.tac = byte,
            _ => unreachable!()
        };
    }

    pub fn read_div(&self) -> u8 {
        self.div
    }

    pub fn reset_div(&mut self) {
        self.div = 0;
    }
}


pub struct Stepper {
    steps_so_far: u32,
    period: u32,
}

impl Stepper {
    /// Iniitalize a Stepper with initial steps and period (units are arbitrary).
    pub fn new(steps_so_far: u32, period: u32) -> Self {
        Stepper {
            steps_so_far,
            period,
        }
    }

    /// Steps through the given steps, returning the number of periods elapsed.
    pub fn step(&mut self, steps: u32) -> u32 {
        let mut steps_to_take = self.steps_so_far + steps;

        let mut periods_elapsed = 0;
        while steps_to_take >= self.period {
            steps_to_take -= self.period;
            periods_elapsed += 1;
        }

        self.steps_so_far = (self.steps_so_far + steps) % self.period;

        return periods_elapsed
    }

    pub fn set_period(&mut self, period: u32) {
        self.period = period;
    }
}