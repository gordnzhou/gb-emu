extern crate sdl2;

mod cpu;
mod bus;
mod ppu;
mod apu;
mod joypad;
mod timer;
mod cartridge;
mod emulator;

use cartridge::Cartridge;
use emulator::Emulator;

use std::io::Write;
use std::fs::OpenOptions;

const ROM_PATH: &str = "roms/pokemonyellow.gbc";

const SCREEN_SCALE: i32 = 5;

// FOR TESTING
#[allow(dead_code)]
fn clear_log_file() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("logs/log.txt")?;
    
    writeln!(file, "")
}

fn main() -> Result<(), String> {

    let cartridge = Cartridge::from_file(ROM_PATH, false);
    let mut emulator = Emulator::new(SCREEN_SCALE, cartridge)?;
    emulator.debug_run(40e12 as u64);

    Ok(())
}
