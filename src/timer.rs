// div is incremented every 256 dots / 64 M-cycles 
const CYCLES_PER_DIV_INC: u32 = 64;

pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,

    next_tma: i32,
    div_cycles: u32,
    tima_cycles: u32,
}

impl Timer {
    pub fn new() -> Self {
        Timer { 
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,

            // number of M-cycles passed after div/tima was incremented
            div_cycles: 0,
            tima_cycles: 0,

            next_tma: -1,
        }
    }

    /// Ticks timer registers over the given period (in cycles); returns true if TIMA overflowed
    pub fn step(&mut self, cycles: u8) -> bool {
        let cycles = cycles as u32;

        self.step_div(cycles);

        if self.tac & 0x04 != 0 {
            self.step_tima(cycles)
        } else {
            false
        }
    }

    fn step_div(&mut self, cycles: u32) {
        if self.div_cycles + cycles >= CYCLES_PER_DIV_INC {
            self.div = self.div.wrapping_add(1);
        }
        self.div_cycles = (self.div_cycles + cycles) % CYCLES_PER_DIV_INC;
    }

    fn step_tima(&mut self, cycles: u32) -> bool {
        let cycles_per_tima_inc: u32 = match self.tac & 0x03 {
            0 => 256,
            1 => 4,
            2 => 16,
            3 => 64,
            _ => unreachable!()
        };

        let mut tima_overflow = false;

        let mut cycles_elapsed = self.tima_cycles + cycles;
        while cycles_elapsed >= cycles_per_tima_inc {
            self.tima = self.tima.wrapping_add(1);
            
            if self.tima == 0 {
                tima_overflow = true;
                self.tima = self.tma;
            }

            cycles_elapsed = cycles_elapsed - cycles_per_tima_inc;
        }
        
        self.tima_cycles = (self.tima_cycles + cycles) % cycles_per_tima_inc;

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
}