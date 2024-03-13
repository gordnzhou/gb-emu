pub struct LengthCounter {
    pub enabled: bool,
    pub ticks: u32,
    max_ticks: u32,
}

impl LengthCounter {
    pub fn new(max_ticks: u32) -> Self {
        LengthCounter {
            enabled: false,
            ticks: 0,
            max_ticks,
        }
    }

    /// Ticks LengthCounter, returning false if length has expired.
    pub fn tick(&mut self) -> bool {
        if self.enabled && self.ticks < self.max_ticks {
            self.ticks += 1;
        }

        self.ticks == self.max_ticks
    }

    pub fn on_trigger(&mut self) {
        if self.ticks == self.max_ticks {
            self.ticks = 0;
        }
    }
}