mod register;
mod cpu;
mod mmu;
mod sdl2;
mod timer;
mod rom;

use cpu::Cpu;

use std::io::Write;
use std::fs::OpenOptions;

const ROM_PATH: &str = "roms/11-op a,(hl).gb";

// FOR TESTING
fn clear_log_file() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("logs/log.txt")?;
    
    writeln!(file, "")
}

fn main() {
    println!("Hello, world!");

    let mut cpu = Cpu::new();
    cpu.load_rom(ROM_PATH);

    // FOR TESTING
    clear_log_file().unwrap();
    let mut lines = 7430000;
    while lines > 0 {
        let _ = cpu.step();
        lines -= 1;
    }
}
