pub struct Sweep {
    pub cur_period: u32,
    shadow_period: u32,
    pub sweep_period: u32,
    pub sweep_up: bool,
    pub shift: u32,

    enabled: bool,
    sweep_ticks: u32,
}

impl Sweep {
    pub fn new() -> Self {
        Sweep {
            cur_period: 0,
            shadow_period: 0,
            sweep_period: 8,
            sweep_up: false,
            shift: 0,  

            enabled: false,
            sweep_ticks: 0,  
        }
    }

    /// Returns false if sweep iteration results in channel being disabled
    pub fn step(&mut self, nr13: &mut u8, nr14: &mut u8) -> bool {
        self.sweep_ticks += 1;

        if self.sweep_ticks == self.sweep_period {
            self.sweep_ticks = if self.sweep_period != 0 {
                0
            } else { 8 };

            if self.enabled && self.sweep_period != 0 {
                let next_period = match self.next_period() {
                    Some(next_period) => next_period,
                    _ => return false, 
                };

                if self.shift > 0 {
                    self.shadow_period = next_period;
                    self.cur_period = next_period;
                    *nr13 = (next_period & 0xFF) as u8;
                    *nr14 = (*nr14 & 0xF8) | ((next_period & 0x700) >> 8) as u8;
                    
                    return self.next_period() == None
                }
            }
        } 

        return true;
    }

    /// Returns next period or None if it will result in an overflow.
    fn next_period(&self) -> Option<u32> {
        let next_period = if self.sweep_up {
            self.shadow_period + self.shadow_period >> self.shift
        } else {
            self.shadow_period - self.shadow_period >> self.shift
        };

        if next_period > 0x7FF {
            None
        } else { Some(next_period) }
    }
    
    /// Returns false if sweep iteration results in channel being disabled
    pub fn on_trigger(&mut self) -> bool {
        self.sweep_ticks = if self.sweep_period != 0 {
            0
        } else { 8 };

        self.shadow_period = self.cur_period;
        self.enabled = self.sweep_period > 0 || self.shift > 0;
        
        if self.shift > 0 {
            return self.next_period() == None;
        } else { true }
    }
}