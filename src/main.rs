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
use cpu::Cpu;
use emulator::Emulator;

const ROM_PATH: &str = "roms/tests/dmg_sound.gb";

const SCREEN_SCALE: i32 = 5;

fn main() -> Result<(), String> {

    let cartridge = Cartridge::from_file(ROM_PATH, false);
    let mut emulator = Emulator::load_cartridge(SCREEN_SCALE, cartridge)?;
    emulator.run_for_duration(40e12 as u64);

    Ok(())
}

#[allow(dead_code)]
fn test_blargg_rom(test_rom_path: &str) {
    let cartridge = Cartridge::from_file(test_rom_path, false);
    let mut cpu = Cpu::new(cartridge);

    let mut cycles: u64 = 0;
    while cycles < (1 << 32) {
        cycles += cpu.step() as u64;

        if cpu.bus.serial_output.contains("Passed") {
            break;
        } else if cpu.bus.serial_output.contains("Failed") {
            panic!("cpu_instr test ROM failed");
        }
    } 
}
