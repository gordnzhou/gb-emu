extern crate sdl2;

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
use cpu::{mooneye_fail_check, mooneye_pass_check, Cpu, GBModel};
use emulator::Emulator;
use config::*;

const ROM_PATH: &str = "roms/tests/cgb-acid2.gbc";

fn main() -> Result<(), String> {

    let cartridge = Cartridge::from_file(ROM_PATH, false);
    let mut emulator = Emulator::load_cartridge(cartridge)?;
    emulator.run_for_duration(40e12 as u64);

    Ok(())
}

const TIMEOUT: u64 = 1 << 32;

#[allow(dead_code)]
fn test_blargg_rom(test_rom_path: &str, model: GBModel) {
    let cartridge = Cartridge::from_file(test_rom_path, false);
    let mut cpu = Cpu::new(cartridge, model);

    let mut cycles: u64 = 0;
    while cycles < TIMEOUT {
        cycles += cpu.step() as u64;

        if cpu.bus.serial_output.contains("Passed") {
            break;
        } else if cpu.bus.serial_output.contains("Failed") {
            panic!("cpu_instr test ROM failed");
        }
    } 
}

#[allow(dead_code)]
fn test_mooneye_rom(test_rom_path: &str, model: GBModel) {
    let cartridge = Cartridge::from_file(test_rom_path, false);
    let mut cpu = Cpu::new(cartridge, model);

    let mut cycles: u64 = 0;
    while cycles < TIMEOUT {
        cycles += cpu.step() as u64;

        if mooneye_fail_check(&cpu) {
            panic!("Mooneye Test Failed: {}", test_rom_path)
        } else if mooneye_pass_check(&cpu) {
            return;
        }
    } 
}
