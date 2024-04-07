

pub struct Joypad {
    joypad: u8,
    interrupt: bool,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            joypad: 0xFF,
            interrupt: false,
        }
    }

    pub fn read_joypad(&self) -> u8 {
        self.joypad
    }

    /// Bottom four bits are read-only.
    pub fn write_joypad(&mut self, byte: u8) {
        let joypad = self.joypad & 0xCF;
        self.joypad = (byte & 0x30) | joypad;
    }

    /// Update joypad register based on status, checking for any interrupts.
    /// status is in order of: START (msb), SELECT, B, A, DOWN, UP, LEFT, RIGHT (lsb).
    /// FALSE/0 = pressed and TRUE/1 = released.
    pub fn update(&mut self, status: u8) {
        self.interrupt = false; 

        let upper_nibble = self.joypad & 0xF0;
        let lower_nibble = if self.select_buttons() && self.select_dpad() {
            (status >> 4) & status & 0xF
        } else {
            if self.select_buttons() {
                status >> 4
            } else if self.select_dpad() {
                status & 0xF
            } else {
                0xF
            }
        };

        if (self.joypad & 0x0F) & !lower_nibble != 0 {
            self.interrupt = true;
        }

        self.joypad = upper_nibble | lower_nibble
    }

    fn select_buttons(&self) -> bool {
        self.joypad & 0x20 == 0
    }

    fn select_dpad(&self) -> bool {
        self.joypad & 0x10 == 0
    }

    pub fn interrupt_triggered(&self) -> bool {
        self.interrupt
    }
}
