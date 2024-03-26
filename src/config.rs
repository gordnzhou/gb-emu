use sdl2::keyboard::Keycode;
use crate::emulator::BYTES_PER_PIXEL;

// in order of: START, SELECT, B, A, DOWN, UP, LEFT, RIGHT.
pub const KEYMAPPINGS: [Keycode; 8] = [
    Keycode::I,
    Keycode::J,
    Keycode::K,
    Keycode::L,
    Keycode::S,
    Keycode::W,
    Keycode::A,
    Keycode::D,
];

pub const SCREEN_SCALE: i32 = 5;

// (CGB ONLY) set to true for display colours to be more 
// realistic to actual hardware
pub const WITH_COLOUR_CORRECTION: bool = true;

///(DMG ONLY)
pub const COLOURS: [[u8; BYTES_PER_PIXEL]; 4] = [
    [0x0F, 0xBC, 0x9B, 0xFF], // => white
    [0x0F, 0xAC, 0x8B, 0xFF], // => light grey
    [0x30, 0x62, 0x30, 0xFF], // => dark grey
    [0x0F, 0x38, 0x0F, 0xFF], // => black
];

pub const DMG_BOOTROM_PATH: &str = "roms/bootrom.gb";

pub const CGB_BOOTROM_PATH: &str = "roms/bootrom.gbc";

pub const SAMPLING_RATE_HZ: u32 = 48000;

pub const AUDIO_SAMPLES: usize = 2048;

pub const MASTER_VOLUME: f32 = 0.2;
