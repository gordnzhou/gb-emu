mod instr;
mod register;

use sdl2::Sdl;

use self::register::Register;
use self::Interrupt::*;

use crate::bus::Bus;
use crate::cartridge::Cartridge;

#[derive(Clone, Copy, Debug)]
pub enum GBModel {
    DMG,
    CGB
}

pub struct Cpu {
    pub bus: Bus,
    model: GBModel,

    pub(self) scheduled_ei: bool,
    pub(self) ime: bool,
    pub(self) halted: bool,
    pub(self) halt_bug: bool,
    pub(self) halt_triggered: bool,
    pub(self) cycles_so_far: u8,

    pub(self) af: Register,
    pub(self) bc: Register,
    pub(self) de: Register,
    pub(self) hl: Register,
    pub(self) pc: Register,
    pub(self) sp: Register,

    // CGB ONLY
    double_speed: bool,
    do_speed_switch: bool,
}

pub enum Interrupt {
    VBlank,
    Stat,
    Timer,
    Serial,
    Joypad,
}

impl Cpu {
    pub fn new(cartridge: Cartridge, model: GBModel) -> Self {
        assert!(!(matches!(model, GBModel::CGB) && !cartridge.cgb_compatible()), 
            "This cartridge is not compatible with CGB functions!");

        if cartridge.has_bootrom() {
            Cpu { 
                bus: Bus::new(cartridge, model),
                model,
                scheduled_ei: false,
                ime: false,
                halted: false,
                halt_bug: false,
                halt_triggered: false,
                cycles_so_far: 0,
                af: Register(0),
                bc: Register(0),
                de: Register(0),
                hl: Register(0),
                pc: Register(0),
                sp: Register(0),
                double_speed: false,
                do_speed_switch: false,
            }
        } else {
            let mut bus = Bus::new(cartridge, model);
            bus.ppu.write_io(0xFF40, 0x91);
            bus.ppu.write_io(0xFF41, 0x81);

            Cpu { 
                bus,
                model,
                scheduled_ei: false,
                ime: false,
                halted: false,
                halt_bug: false,
                halt_triggered: false,
                cycles_so_far: 0,
                af: Register(0x01B0),
                bc: Register(0x0013),
                de: Register(0x00D8),
                hl: Register(0x014D),
                pc: Register(0x0100),
                sp: Register(0xFFFE),
                double_speed: false,
                do_speed_switch: false
            }
        }
    }

    pub fn with_audio(mut self, sdl: Sdl) -> Self {
        self.bus = self.bus.with_audio(sdl);
        self
    }

    /// Steps through all parts of the emulator over the period
    /// that the next CPU instruction will take; returns that period's length in M-cycles.
    pub fn step(&mut self) -> u8 {
        let mut cycles = self.cycle();
        if self.double_speed {
            cycles += self.cycle();
            cycles /= 2;
        }

        if matches!(self.model, GBModel::CGB) && self.do_speed_switch {
            self.do_speed_switch = false;
            self.double_speed = !self.double_speed;
            // TODO: implement pausing after STOP instruction triggers speed switch
            return 200 
        }

        self.bus.step(cycles);
       
        cycles
    }

    /// Do a CPU fetch-execute cycle and return the number of clock M-cycles taken.
    fn cycle(&mut self) -> u8 {
        self.halt_triggered = false;

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
                } else if self.halt_triggered {
                    self.halt_bug = true;
                }

                self.ime = false;
                self.halted = false;
            },
            None => {}
        }

        if cycles > self.cycles_so_far {
            self.bus.partial_step(cycles - self.cycles_so_far);
        }
        self.cycles_so_far = 0;

        cycles
    }
    
    /// Returns the next pending interrupt by priority
    fn get_pending_interrupt(&mut self) -> Option<Interrupt>{
        let interrupt_enable: u8 =  self.bus.read_byte(0xFFFF);
        let interrupt_flag: u8 = self.bus.read_byte(0xFF0F);

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
    use crate::test_blargg_rom;

    const CPU_INSTR: &str = "roms/tests/cpu_instrs.gb";
    const MEM_TIMING: &str = "roms/tests/mem_timing.gb";
    const INSTR_TIMING: &str = "roms/tests/instr_timing.gb";

    #[test]
    fn cpu_instr_test() {
        test_blargg_rom(CPU_INSTR, super::GBModel::DMG);
    }

    #[test]
    fn cpu_mem_timing_test() {
        test_blargg_rom(MEM_TIMING, super::GBModel::DMG);
    }

    #[test]
    fn cpu_instr_timing_test() {
        test_blargg_rom(INSTR_TIMING, super::GBModel::DMG);
    }
}

pub fn mooneye_pass_check(cpu: &Cpu) -> bool {
    cpu.bc.hi() == 3 && 
    cpu.bc.lo() == 5 && 
    cpu.de.hi() == 8 && 
    cpu.de.lo() == 13 &&
    cpu.hl.hi() == 21 && 
    cpu.hl.lo() == 34
}  

pub fn mooneye_fail_check(cpu: &Cpu) -> bool {
    cpu.bc.hi() == 0x42 && 
    cpu.bc.lo() == 0x42 && 
    cpu.de.hi() == 0x42 && 
    cpu.de.lo() == 0x42 &&
    cpu.hl.hi() == 0x42 && 
    cpu.hl.lo() == 0x42
}