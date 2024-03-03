extern crate sdl2;

mod cpu;
mod bus;
mod ppu;
mod apu;
mod joypad;
mod timer;
mod memory;
mod emulator;

use emulator::Emulator;

use std::io::Write;
use std::fs::OpenOptions;

const ROM_PATH: &str = "roms/halt_bug.gb";

const SCREEN_SCALE: i32 = 3;

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
    // clear_log_file().unwrap();

    let mut emulator = Emulator::new(SCREEN_SCALE, ROM_PATH, true)?;
    emulator.debug_run(40e9 as u64);

    Ok(())
}
