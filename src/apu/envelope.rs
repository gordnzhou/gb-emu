pub struct Envelope {
    fs_ticks: u8,
    cur_volume: u8,
    sweep_pace: u8, 
    initial_volume: u8,
    envelope_up: bool,
    sweep_ticks: u8,
}

impl Envelope {
    pub fn new() -> Self {
        Envelope {
            fs_ticks: 0,
            cur_volume: 0,
            sweep_ticks: 0,
            sweep_pace: 0,
            initial_volume: 0,
            envelope_up: false,
        }
    }

    pub fn step(&mut self) {
        self.fs_ticks = self.fs_ticks.wrapping_add(1);
        if self.fs_ticks % 8 == 0 {

            if self.sweep_pace == 0 {
                return;
            }
    
            self.sweep_ticks += 1;
    
            if self.sweep_ticks == self.sweep_pace {
                self.sweep_ticks = 0;
    
                let next_volume = if self.envelope_up {
                    self.cur_volume as i8 + 1
                } else {
                    self.cur_volume as i8 - 1
                };
    
                if 0x0 <= next_volume && next_volume <= 0xF {
                    self.cur_volume = next_volume as u8;
                }
            }
        }
    }
    
    pub fn on_trigger(&mut self) {
        self.sweep_ticks = 0;
        self.cur_volume = self.initial_volume;
    }

    pub fn set(&mut self, byte: u8) {
        self.initial_volume = (byte & 0xF0) >> 4;
        self.envelope_up = byte & 8 != 0;
        self.sweep_pace = byte & 7;
    }

    pub fn volume(&self) -> u8 {
        self.cur_volume
    }
}

