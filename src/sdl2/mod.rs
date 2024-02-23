mod joypad;
mod apu;
mod ppu;

use joypad::Joypad;
use apu::Apu;
use ppu::Ppu;

pub struct Sdl2Wrapper {
    pub joypad: Joypad,
    pub apu: Apu,
    pub ppu: Ppu,
}

impl Sdl2Wrapper {
    pub fn new() -> Self {
        Sdl2Wrapper {
            joypad: Joypad::new(),
            apu: Apu::new(),
            ppu: Ppu::new(),
        }
    }

    pub fn step(&mut self) {
        self.ppu.step();
        self.apu.step();
        self.joypad.step();
    }
}