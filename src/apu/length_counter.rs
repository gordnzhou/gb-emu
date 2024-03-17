pub struct LengthCounter {
    fs_ticks: u8,
    pub enabled: bool,
    pub ticks: u32,
    max_ticks: u32,
}

impl LengthCounter {
    pub fn new(max_ticks: u32) -> Self {
        LengthCounter {
            fs_ticks: 1,
            enabled: false,
            ticks: max_ticks,
            max_ticks,
        }
    }

    /// Ticks LengthCounter, returning true if length has expired.
    pub fn tick(&mut self, div_apu_tick: u8) -> bool {
        self.fs_ticks += div_apu_tick;
        if self.fs_ticks < 2 { return self.ticks == 0 }
        self.fs_ticks = 0;
        
        if self.enabled && self.ticks > 0 {
            self.ticks -= 1;
        }

        self.ticks == 0
    }

    pub fn set_ticks(&mut self, value: u32) {
        self.ticks = self.max_ticks - value;
    }

    pub fn on_trigger(&mut self) {
        if self.ticks == 0 {
            self.ticks = self.max_ticks;
        }
    }

    /// SOURCE: https://gbdev.gg8.se/wiki/articles/Gameboy_sound_hardware (under "Obscure Behavior")
    pub fn extra_clocking(&mut self, new_enabled: bool) -> bool {
        if self.fs_ticks == 0 && !self.enabled && new_enabled && self.ticks > 0 {
            self.ticks -= 1;
            if self.ticks == 0 {
                return false;
            }
        }

        true
    }
}