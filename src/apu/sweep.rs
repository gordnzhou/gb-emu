pub struct Sweep {
    fs_ticks: u8,
    cur_freq_period: u32,
    shadow_freq_period: u32,
    sweep_period: u32,
    sweep_down: bool,
    shift: u32,
    enabled: bool,
    sweep_timer: u32,
    sweep_down_calc: bool
}

impl Sweep {
    pub fn new() -> Self {
        Sweep {
            fs_ticks: 1,
            cur_freq_period: 0,
            shadow_freq_period: 0,
            sweep_period: 8,
            sweep_down: true,
            shift: 0,  
            enabled: false,
            sweep_timer: 0, 
            sweep_down_calc: false, 
        }
    }

    /// Returns false if sweep iteration results in channel being disabled
    pub fn step(&mut self, nr13: &mut u8, nr14: &mut u8) -> bool {
        self.fs_ticks = self.fs_ticks.wrapping_add(1);
        if self.fs_ticks % 4 == 0 {
            
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
        }

        true
    }
    
    /// Returns false if sweep iteration results in channel being disabled
    pub fn on_trigger(&mut self) -> bool {
        self.sweep_timer = if self.sweep_period > 0 {
            self.sweep_period
        } else { 8 };

        self.sweep_down_calc = false;
        self.shadow_freq_period = self.cur_freq_period;
        self.enabled = self.sweep_period > 0 || self.shift > 0;
        
        if self.shift > 0 {
            return self.next_freq_period().is_some();
        } else { true }
    }

    /// Returns next period or None if it will result in an overflow.
    fn next_freq_period(&mut self) -> Option<u32> {
        let next_period = if self.sweep_down {
            self.sweep_down_calc = true;
            self.shadow_freq_period - (self.shadow_freq_period >> self.shift)
        } else {
            self.shadow_freq_period + (self.shadow_freq_period >> self.shift)
        };

        if next_period > 0x7FF {
            None
        } else { Some(next_period) }
    }

    /// Sets sweep fields based on NR10 registers; returns false if channel is disabled.
    pub fn set(&mut self, byte: u8) -> bool {
        let old_sweep_down = self.sweep_down;

        self.shift = byte as u32 & 0x07;
        self.sweep_down = byte & 0x08 != 0;
        self.sweep_period = (byte as u32 & 0x70) >> 4;

        // Obscure Behavior: setting sweep direciton from - to + after any sweep calculation has
        // done made since last trigger, disabled channel. 
        if old_sweep_down && !self.sweep_down && self.sweep_down_calc {
            return false;
        }
        if !old_sweep_down && self.sweep_down {
            self.sweep_down_calc = false;
        }

        true
    }

    /// Set this everytime NR13 or NR14 get updated
    pub fn set_period(&mut self, period: u32) {
        self.cur_freq_period = period;
    }
}