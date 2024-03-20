pub struct Sweep {
    fs_ticks: u8,
    cur_freq_period: u32,
    shadow_freq_period: u32,
    sweep_period: u32,
    sweep_up: bool,
    shift: u32,
    enabled: bool,
    sweep_timer: u32,
}

impl Sweep {
    pub fn new() -> Self {
        Sweep {
            fs_ticks: 1,
            cur_freq_period: 0,
            shadow_freq_period: 0,
            sweep_period: 8,
            sweep_up: false,
            shift: 0,  
            enabled: false,
            sweep_timer: 0,  
        }
    }

    /// Returns false if sweep iteration results in channel being disabled
    pub fn step(&mut self, div_apu_tick: u8, nr13: &mut u8, nr14: &mut u8) -> bool {
        self.fs_ticks += div_apu_tick;
        if self.fs_ticks < 4 { return true; }
        self.fs_ticks = 0;

        if self.sweep_timer > 0 {
            self.sweep_timer -= 1;
        }

        if self.sweep_timer == 0 {
            self.sweep_timer = if self.sweep_period > 0 {
                self.sweep_period
            } else { 8 };

            if self.enabled && self.sweep_period > 0 {
                let next_freq_period = match self.next_freq_period() {
                    Some(next_freq_period) => next_freq_period,
                    None => return false, 
                };

                if self.shift > 0 {
                    self.shadow_freq_period = next_freq_period;
                    self.cur_freq_period = next_freq_period;
                    *nr13 = (next_freq_period & 0xFF) as u8;
                    *nr14 = (*nr14 & 0xF8) | ((next_freq_period & 0x700) >> 8) as u8;
                    
                    return self.next_freq_period().is_some();
                }
            }
        } 

        true
    }

    /// Returns next period or None if it will result in an overflow.
    fn next_freq_period(&self) -> Option<u32> {
        let next_period = if self.sweep_up {
            self.shadow_freq_period + (self.shadow_freq_period >> self.shift)
        } else {
            self.shadow_freq_period - (self.shadow_freq_period >> self.shift)
        };

        if next_period > 0x7FF {
            None
        } else { Some(next_period) }
    }
    
    /// Returns false if sweep iteration results in channel being disabled
    pub fn on_trigger(&mut self) -> bool {
        self.sweep_timer = if self.sweep_period > 0 {
            self.sweep_period
        } else { 8 };

        self.shadow_freq_period = self.cur_freq_period;
        self.enabled = self.sweep_period > 0 || self.shift > 0;
        
        if self.shift > 0 {
            return self.next_freq_period().is_some();
        } else { true }
    }

    pub fn set(&mut self, byte: u8) {
        self.shift = byte as u32 & 0x07;
        self.sweep_up = byte & 0x08 == 0;
        self.sweep_period = (byte as u32 & 0x70) >> 4;
    }

    /// Set this everytime NR13 or NR14 get updated
    pub fn set_period(&mut self, period: u32) {
        self.cur_freq_period = period
    }
}