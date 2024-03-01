

pub struct Joypad {
    joypad: u8,
    interrupt: bool,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            joypad: 0,
            interrupt: false,
        }
    }

    pub fn read_joypad(&self) -> u8 {
        self.joypad
    }

    /// Bottom four bits are read-only.
    pub fn write_joypad(&mut self, byte: u8) {
        let joypad = self.joypad & 0x0F;
        self.joypad = (byte & 0xF0) | joypad;
    }

    /// Update joypad register based on key_status, checking for any interrupts.
    /// key_status is in order of: START, SELECT, B, A, DOWN, UP, LEFT, RIGHT.
    /// FALSE/0 = pressed and TRUE/1 = released.
    pub fn step(&mut self, key_status: [bool; 8]) {
        self.interrupt = false; 

        let mut status = 0;
        for bit in key_status {
            status <<= 1;
            status |= bit as u8;
        }

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

        self.joypad = upper_nibble | lower_nibble
    }

    fn select_buttons(&self) -> bool {
        self.joypad & 0x20 == 0
    }

    fn select_dpad(&self) -> bool {
        self.joypad & 0x10 == 0
    }

    // fn set_bit(&mut self, k: usize, val: bool) {
    //     if val {
    //         self.0 |= 1 << k;
    //     } else {
    //         self.0 &= !(1 << k);
    //     }
    // }
}
