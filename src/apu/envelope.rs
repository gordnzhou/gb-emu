pub struct Envelope {
    fs_ticks: u8,
    pub cur_volume: u8,
    pub sweep_pace: u8, 
    pub initial_volume: u8,
    pub envelope_up: bool,
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

    pub fn step(&mut self, div_apu_tick: u8) {
        self.fs_ticks += div_apu_tick;
        if self.fs_ticks < 8 { return; }
        self.fs_ticks = 0;

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
    
    pub fn on_trigger(&mut self) {
        self.sweep_ticks = 0;
        self.cur_volume = self.initial_volume;
    }
}

