mod register;
mod cpu;
mod mmu;
mod sdl2;
mod timer;
mod rom;

use cpu::Sm83;
use std::io::{stdin, stdout, Read, Write};
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

    let mut sm83 = Sm83::new();
    sm83.load_rom(ROM_PATH);

    // FOR TESTING
    clear_log_file().unwrap();
    let mut lines = 7430000;
    while lines > 0 {
        let _ = sm83.fetch_execute();
        lines -= 1;
    }
}
