mod instr;

use std::fs::File;
use std::io::{self, Read};

use crate::mmu::Mmu;
use crate::register::Register;

pub struct Cpu {
    pub memory: Mmu,

    pub(self) ime: bool,
    pub(self) halted: bool,
    pub(self) af: Register,
    pub(self) bc: Register,
    pub(self) de: Register,
    pub(self) hl: Register,
    pub(self) pc: Register,
    pub(self) sp: Register,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { 
            af: Register(0x01B0),
            bc: Register(0x0013),
            de: Register(0x00D8),
            hl: Register(0x014D),
            pc: Register(0x0100),
            sp: Register(0xFFFE),
            ime: false,
            halted: false,
            memory: Mmu::new(),
        }
    }

    /// Returns the number of clock M-cycles.
    pub fn step(&mut self) -> u8 {
        self.memory.sdl2_wrapper.step();
        self.execute_next_instruction()
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        match Cpu::read_rom_from_file(rom_path) {
            Ok(rom_data) => {
                for i in 0..rom_data.len() {
                    self.memory.write_byte(i as u16, rom_data[i]);
                }

                println!("Sucessfully read ROM starting at memory address {:#06x}", 0);
                println!("ROM size: {} bytes", rom_data.len());
                println!("First bytes: {:?}", &rom_data[..16]);
            }
            Err(err) => {
                eprintln!("Error reading ROM file: {}", err);
            }
        }
    }

    // TODO: MOVE TO MMU
    fn read_rom_from_file(file_path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(file_path)?;

        let mut rom_data = Vec::new();
        file.read_to_end(&mut rom_data)?;

        Ok(rom_data)
    }
}