pub struct Register(pub usize);

impl Register {
    pub fn full(&self) -> usize {
        self.0
    }

    pub fn hi(&self) -> u8 {
        ((self.0 & 0xFF00) >> 8) as u8
    }

    pub fn lo(&self) -> u8 {
        self.0 as u8 & 0xFF
    }

    pub fn bit(&self, k: usize) -> bool {
        self.0 & (1 << k) != 0
    }

    pub fn set(&mut self, val: usize) {
        self.0 = val;
    }

    pub fn set_hi(&mut self, val: u8) {
        self.0 = ((val as usize) << 8) + self.lo() as usize;
    }

    pub fn set_lo(&mut self, val: u8) {
        self.0 = ((self.hi() as usize) << 8) + val as usize;
    }

    pub fn set_bit(&mut self, k: usize, val: bool) {
        if val {
            self.0 |= 1 << k;
        } else {
            self.0 &= !(1 << k);
        }
    }
}