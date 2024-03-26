pub struct LengthCounter {
    fs_ticks: u8,
    channel_on: bool,
    enabled: bool,
    ticks: u32,
    max_ticks: u32,
}

impl LengthCounter {
    pub fn new(max_ticks: u32) -> Self {
        LengthCounter {
            fs_ticks: 1,
            channel_on: false,
            enabled: false,
            ticks: max_ticks,
            max_ticks,
        }
    }

    /// Ticks LengthCounter, updating channel status if length expires
    pub fn tick(&mut self) {
        self.fs_ticks = self.fs_ticks.wrapping_add(1);
        if self.fs_ticks % 2 == 0 {

            if self.enabled && self.ticks > 0 {
                self.ticks -= 1;
                
                if self.ticks == 0 {
                    self.channel_on = false;
                }
            }
        } 
    }

    pub fn set_ticks(&mut self, byte: u8) {
        self.ticks = self.max_ticks - ((byte as u32) & (self.max_ticks - 1));
    }

    pub fn on_trigger(&mut self) {
        self.channel_on = true;
        if self.ticks == 0 {
            self.ticks = self.max_ticks;

            // Obscure Behavior: On a trigger, if the frame sequencer's next tick 
            // does not clock the counter and length enable bit is set, reloads the 
            // counter with MAX_TICKS - 1 instead of MAX_TICKS
            if self.fs_ticks % 2 == 0 && self.enabled {
                self.ticks = self.max_ticks - 1;
            }
        }
    }

    /// Obscure Behavior: If the frame sequencer's next tick does not clock the counter and
    /// length enable bit goes from 0 to 1, decrements counter if counter is not zero; 
    /// channel is disabled if counter reached zero
    pub fn extra_clocking(&mut self, new_enabled: bool){
        if self.ticks > 0 && self.fs_ticks % 2 == 0 && !self.enabled && new_enabled {
            self.ticks -= 1;

            if self.ticks == 0 {
                self.channel_on = false;
            }
        }
    }

    pub fn turn_off_channel(&mut self) {
        self.channel_on = false;
    }

    pub fn set_tick_enable(&mut self, enable: bool) {
        self.enabled = enable
    }

    pub fn channel_on(&self) -> bool {
        self.channel_on
    }
}