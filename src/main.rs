#[cfg(not(target_arch = "wasm32"))]
extern crate sdl2;

extern crate gbemulib;

mod cpu;
mod config;
mod bus;
mod ppu;
mod apu;
mod joypad;
mod timer;
mod cartridge;
mod emulator;

use cartridge::Cartridge;
use emulator::Emulator;
use gbemulib::constants;

const ROM_PATH: &str = "roms/pokemoncrystal.gbc";
const WITH_BOOTROM: bool = true;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), String> {
    
    let cartridge = Cartridge::from_file(ROM_PATH, WITH_BOOTROM);
    let mut emulator = Emulator::load_cartridge(cartridge)?;
    emulator.run_for_duration(40e12 as u64);

    Ok(())
}