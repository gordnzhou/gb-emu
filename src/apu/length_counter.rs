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
    pub fn tick(&mut self, div_apu_tick: u8) {
        self.fs_ticks = self.fs_ticks.wrapping_add(div_apu_tick);
        if self.fs_ticks % 2 == 0 {

            if self.enabled && self.ticks > 0 {
                self.ticks -= 1;
                self.channel_on = self.ticks == 0;
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
        }
    }

    /// SOURCE: https://gbdev.gg8.se/wiki/articles/Gameboy_sound_hardware (under "Obscure Behavior")
    pub fn extra_clocking(&mut self, new_enabled: bool){
        if self.fs_ticks == 0 && !self.enabled && new_enabled && self.ticks > 0 {
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