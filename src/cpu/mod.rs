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

enum Interrupt {
    VBlank,
    Stat,
    Timer,
    Serial,
    Joypad,
}

impl Cpu {
    pub fn new(memory: Mmu) -> Self {
        Cpu { 
            memory,
            scheduled_ei: false,
            ime: false,
            halted: false,
            halt_bug: false,
            af: Register(0x01B0),
            bc: Register(0x0013),
            de: Register(0x00D8),
            hl: Register(0x014D),
            pc: Register(0x0100),
            sp: Register(0xFFFE),
        }
    }

    /// Steps through all parts of the emulator over the period
    /// that the next CPU instruction will take; returns the period length in M-cycles
    pub fn step(&mut self) -> u8 {
        let cycles = self.cycle();
        self.memory.sdl2_wrapper.step();
        cycles
    }

    /// Do a CPU fetch-execute cycle and return the number of clock M-cycles taken.
    fn cycle(&mut self) -> u8 {
        if self.scheduled_ei {
            self.ime = true;
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
    
    /// Returns the next pending interrupt by priority; if one is found, resets its corresponding bit in IE.
    fn get_pending_interrupt(&mut self) -> Option<Interrupt>{
        let interrupt_enable: u8 =  self.memory.read_byte(0xFFFF);
        let interrupt_flag: u8 = self.memory.read_byte(0xFF0F);

        for bit in 0..=4 {   
            if (interrupt_enable & (1 << bit) != 0) && (interrupt_flag & (1 << bit) != 0) {
                self.memory.write_byte(0xFFFF, interrupt_enable & !(1 << bit));

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