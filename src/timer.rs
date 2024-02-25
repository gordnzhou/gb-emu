// div is incremented every 256 dots / 64 M-cycles 
const CYCLES_PER_DIV_INC: u32 = 64;

pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,

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

            div_cycles: 0,
            tima_cycles: 0,
        }
    }

    /// Ticks timer registers over cycles period; return true if TIMA overflowed
    pub fn step(&mut self, cycles: u8) -> bool {
        let cycles = cycles as u32;

        if self.div_cycles + cycles >= CYCLES_PER_DIV_INC {
            self.div = self.div.wrapping_add(1);
        }
        self.div_cycles = (self.div_cycles + cycles) % CYCLES_PER_DIV_INC;

        if self.tac & 0x04 == 0 {
            return false;
        }

        let cycles_per_tima_inc: u32 = match self.tac & 0x03 {
            0 => 256,
            1 => 4,
            2 => 16,
            3 => 64,
            _ => unreachable!()
        };

        let mut tima_overflow = false;
        
        if self.tima_cycles + cycles >= cycles_per_tima_inc {
            self.div = self.div.wrapping_add(1);
            
            if self.div == 0 {
                tima_overflow = true;
                self.div = self.tma;
            }
        }
        self.tima_cycles = (self.tima_cycles + cycles) % cycles_per_tima_inc;

        tima_overflow

    }

    pub fn read_div(&self) -> u8 {
        self.div
    }

    /// Writing to div sets it to 0x00.
    pub fn write_div(&mut self, _byte: u8) {
        self.div = 0x00;
    }

    pub fn read_tima(&self) -> u8 {
        self.tima
    }

    pub fn write_tima(&mut self, byte: u8) {
        self.tima = byte;
    }

    pub fn read_tma(&self) -> u8 {
        self.tma
    }

    pub fn write_tma(&mut self, byte: u8) {
        self.tma = byte;
    }

    pub fn read_tac(&self) -> u8 {
        self.tac
    }

    pub fn write_tac(&mut self, byte: u8) {
        self.tac = byte;
    }
}