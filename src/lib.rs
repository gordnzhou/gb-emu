extern crate wasm_bindgen;

mod cpu;
mod config;
mod bus;
mod ppu;
mod apu;
mod joypad;
mod timer;
mod cartridge;

pub use cartridge::Cartridge;
use config::AUDIO_SAMPLES;
use constants::{BYTES_PER_PIXEL, LCD_BYTE_WIDTH, LCD_HEIGHT};
pub use cpu::Cpu;

use cpu::GBModel;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub mod constants {
    pub const LCD_WIDTH: usize= 160;
    pub const LCD_HEIGHT: usize = 144;

    pub const BYTES_PER_PIXEL: usize = 4;

    pub const LCD_BYTE_WIDTH: usize = BYTES_PER_PIXEL * LCD_WIDTH;

    // DETERMINES GAME SPEED
    pub const T_CYCLE_HZ: u32 = 1 << 22;
    pub const M_CYCLE_HZ: u32 = T_CYCLE_HZ >> 2;

    // 1 T Cycle = 2^22 Hz = 1/4 M-cycle = 238.4... ns
    pub const T_CYCLE_DURATION_NS: u64 = (1e9 as u32 / T_CYCLE_HZ) as u64;
}

#[wasm_bindgen]
pub struct Emulator {
    cpu: Cpu,
}

#[wasm_bindgen]
impl Emulator {
    pub fn new(cartridge_bytes: &[u8]) -> Self {
        let cartridge = Cartridge::from_bytes(cartridge_bytes);

        let model = if cartridge.cgb_compatible() {
            GBModel::CGB
        } else {
            GBModel::DMG
        };
        log(&format!("detected model: {:?}", model));

        Emulator { 
            cpu: Cpu::new(cartridge, model),
        }

    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn get_audio_output(&mut self) -> Option<*const [f32; 2]> {
        match self.cpu.get_audio_output() {
            Some(audio_output) => Some(audio_output.as_ptr()),
            None => None
        }
    }

    pub fn audio_output_length(&self) -> usize {
        AUDIO_SAMPLES
    }

    pub fn get_display_output(&mut self) -> Option<*const u8> {
        match self.cpu.get_display_output() {
            Some(display_output) => Some(display_output.as_ptr()),
            None => None
        }
    }

    pub fn display_height(&self) -> usize {
        LCD_HEIGHT
    }

    pub fn display_width(&self) -> usize {
        LCD_BYTE_WIDTH
    }

    pub fn display_byte_length(&self) -> usize {
        BYTES_PER_PIXEL
    }

    pub fn entered_hblank(&self) -> bool {
        self.cpu.entered_hblank()
    }

    pub fn update_joypad(&mut self, status: u8) {
        self.cpu.update_joypad(status)
    }
}
