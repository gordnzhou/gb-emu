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
use config::{AUDIO_SAMPLES, SAMPLING_RATE_HZ};
use constants::{BYTES_PER_PIXEL, LCD_HEIGHT, LCD_WIDTH};
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
    title: String,
    cpu: Cpu,
    audio_output_flat: [f32; 2 * AUDIO_SAMPLES],
}

#[wasm_bindgen]
impl Emulator {
    pub fn new(cartridge_bytes: &[u8]) -> Self {
        let cartridge = Cartridge::from_bytes(cartridge_bytes);
        let title = cartridge.get_title();

        let model = if cartridge.cgb_compatible() {
            GBModel::CGB
        } else {
            GBModel::DMG
        };
        log(&format!("detected model: {:?}", model));

        Emulator { 
            title,
            cpu: Cpu::new(cartridge, model),
            audio_output_flat: [0.0; 2 * AUDIO_SAMPLES],
        }
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn game_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_audio_output(&mut self) -> Option<*const f32> {
        match self.cpu.get_audio_output() {
            Some(audio_output) => {
                for i in 0..AUDIO_SAMPLES {
                    self.audio_output_flat[2 * i] = audio_output[i][0];
                    self.audio_output_flat[2 * i + 1] = audio_output[i][1];
                }
                Some(self.audio_output_flat.as_ptr())
            },
            None => None
        }
    }

    pub fn audio_output_length() -> usize {
        2 * AUDIO_SAMPLES
    }

    pub fn audio_rate() -> u32 {
        SAMPLING_RATE_HZ
    }

    pub fn get_display_output(&mut self) -> Option<*const u8> {
        match self.cpu.get_display_output() {
            Some(display_output) => Some(display_output.as_ptr()),
            None => None
        }
    }

    pub fn display_height() -> usize {
        LCD_HEIGHT
    }

    pub fn display_width() -> usize {
        LCD_WIDTH
    }

    pub fn display_byte_length() -> usize {
        BYTES_PER_PIXEL
    }

    pub fn entered_hblank(&self) -> bool {
        self.cpu.entered_hblank()
    }

    pub fn update_joypad(&mut self, status: u8) {
        self.cpu.update_joypad(status)
    }
}
