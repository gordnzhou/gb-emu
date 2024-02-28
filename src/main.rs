extern crate sdl2;

mod register;
mod cpu;
mod mmu;
mod ppu;
mod apu;
mod joypad;
mod timer;
mod rom;
mod emulator;

use emulator::Emulator;

const ROM_PATH: &str = "roms/10-bit ops.gb";
const SCREEN_SCALE: i32 = 3;

fn main() -> Result<(), String> {

    let mut emulator = Emulator::new(SCREEN_SCALE)?;
    emulator.load_rom(ROM_PATH);
    emulator.debug_run(5e9 as u64);

    Ok(())
}
