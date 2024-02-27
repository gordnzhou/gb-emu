extern crate sdl2;

mod register;
mod cpu;
mod mmu;
mod ui;
mod timer;
mod rom;
mod emulator;

use cpu::Cpu;
use emulator::Emulator;
use mmu::Mmu;
use ui::Sdl2Wrapper;

use std::io::Write;
use std::fs::OpenOptions;

const ROM_PATH: &str = "roms/02-interrupts.gb";
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
    // clear_log_file();

    let sdl2_wrapper = Sdl2Wrapper::new(SCREEN_SCALE)?;

    let mut mmu = Mmu::new(sdl2_wrapper);
    mmu.load_rom(ROM_PATH);

    let cpu = Cpu::new(mmu);

    let mut emulator = Emulator::new(cpu);

    emulator.debug_run(20e9 as u64);

    Ok(())
}
