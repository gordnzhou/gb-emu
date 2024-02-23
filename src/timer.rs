pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new() -> Self {
        Timer { 
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

    pub fn read_div(&self) -> u8 {
        self.div
    }

    pub fn write_div(&mut self, byte: u8) {
        self.div = byte;
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