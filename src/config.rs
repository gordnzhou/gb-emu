use crate::constants::BYTES_PER_PIXEL;

// (CGB ONLY) set to true for display colours to be more 
// realistic to actual hardware
pub const WITH_COLOUR_CORRECTION: bool = true;

///(DMG ONLY)
pub const COLOURS: [[u8; BYTES_PER_PIXEL]; 4] = [
    [0xE8, 0xFF, 0xFF, 0xFF], // => white
    [0x74, 0xD4, 0x9B, 0xFF], // => light grey
    [0x80, 0x9A, 0x30, 0xFF], // => dark grey
    [0x4F, 0x3D, 0x1A, 0xFF], // => black
];

pub const DMG_BOOTROM_PATH: &str = "bootroms/bootrom.gb";

pub const CGB_BOOTROM_PATH: &str = "bootroms/bootrom.gbc";

pub const SAMPLING_RATE_HZ: u32 = 48000;

pub const AUDIO_SAMPLES: usize = 2048;
