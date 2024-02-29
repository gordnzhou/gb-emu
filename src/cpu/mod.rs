mod instr;

use crate::mmu::Mmu;
use crate::register::Register;
use crate::cpu::Interrupt::*;

pub struct Cpu {
    pub memory: Mmu,

    pub(self) scheduled_ei: bool,
    pub(self) ime: bool,
    pub(self) halted: bool,
    pub(self) halt_bug: bool,

    pub(self) af: Register,
    pub(self) bc: Register,
    pub(self) de: Register,
    pub(self) hl: Register,
    pub(self) pc: Register,
    pub(self) sp: Register,
}

pub enum Interrupt {
    VBlank,
    Stat,
    Timer,
    Serial,
    Joypad,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { 
            memory: Mmu::new(),
            scheduled_ei: false,
            ime: false,
            halted: false,
            halt_bug: false,
            af: Register(0x0000),
            bc: Register(0x0000),
            de: Register(0x0000),
            hl: Register(0x0000),
            pc: Register(0x0000), // switch to 0x100 to skip bootrom
            sp: Register(0x0000),
        }
    }

    /// Steps through all parts of the emulator over the period
    /// that the next CPU instruction will take; returns that period's length in M-cycles.
    pub fn step(&mut self) -> u8 {
        let cycles = self.cycle();

        self.memory.step(cycles);
        
        cycles
    }

    /// Do a CPU fetch-execute cycle and return the number of clock M-cycles taken.
    fn cycle(&mut self) -> u8 {
        if self.scheduled_ei {
            self.ime = true;
            self.scheduled_ei = false;
        }
        
        let mut cycles = if !self.halted {
            self.execute_next_instruction()
        } else {
            1
        };

        match self.get_pending_interrupt() {
            Some(interrupt) => {
                if self.ime {
                    cycles += self.handle_interrupt(interrupt);
                } else if self.halted {
                    self.halt_bug = true;
                }

                self.ime = false;
                self.halted = false;
            },
            None => {}
        }

        cycles
    }
    
    /// Returns the next pending interrupt by priority
    fn get_pending_interrupt(&mut self) -> Option<Interrupt>{
        let interrupt_enable: u8 =  self.memory.read_byte(0xFFFF);
        let interrupt_flag: u8 = self.memory.read_byte(0xFF0F);

        for bit in 0..=4 {   
            if interrupt_enable & (1 << bit) != 0 && interrupt_flag & (1 << bit) != 0 {

                let interrupt: Interrupt = match bit {
                    0 => VBlank,
                    1 => Stat,
                    2 => Timer,
                    3 => Serial,
                    4 => Joypad,
                    _ => unreachable!()
                };

                return Some(interrupt)
            }
        }   
        
        None
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use crate::cpu::Cpu;

    const TIMEOUT: u64 = 1 << 32;
    const TEST_FILES: [&str; 11] = [
        "01-special.gb",
        "02-interrupts.gb",
        "03-op sp,hl.gb",
        "04-op r,imm.gb",
        "05-op rp.gb",
        "06-ld r,r.gb",
        "07-jr,jp,call,ret,rst.gb",
        "08-misc instrs.gb",
        "09-op r,r.gb",
        "10-bit ops.gb",
        "11-op a,(hl).gb"
    ];

    #[test]
    fn cpu_instr_test() {
        'outer: for test in TEST_FILES {
            let mut cpu = Cpu::new();
            cpu.memory.load_rom(&*format!("roms/{}", test));

            let mut cycles: u64 = 0;
            while cycles < TIMEOUT {
                cycles += cpu.step() as u64;

                if cpu.memory.serial_output.contains("Passed") {
                    continue 'outer
                } else if cpu.memory.serial_output.contains("Failed") {
                    break;
                }
            } 
            panic!("test rom failed: {}", test);
        }
    }
}