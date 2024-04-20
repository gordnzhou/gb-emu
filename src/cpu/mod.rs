mod instr;
mod register;

use self::register::Register;
use self::Interrupt::*;

use crate::bus::Bus;
use crate::cartridge::Cartridge;
use crate::config::AUDIO_SAMPLES;
use crate::constants::{LCD_BYTE_WIDTH, LCD_HEIGHT};

#[derive(Clone, Copy, Debug)]
pub enum GBModel {
    DMG,
    CGB
}

pub struct Cpu {
    bus: Bus,
    model: GBModel,

    pub(self) scheduled_ei: bool,
    pub(self) ime: bool,
    pub(self) halted: bool,
    pub(self) halt_bug: bool,
    pub(self) halt_triggered: bool,
    pub(self) t_cycles_so_far: u32,

    pub(self) af: Register,
    pub(self) bc: Register,
    pub(self) de: Register,
    pub(self) hl: Register,
    pub(self) pc: Register,
    pub(self) sp: Register,

    // CGB ONLY
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
            let bus = Bus::new(cartridge, model);
            Cpu::make_cpu(0, 0, 00, 0, 0, 0, model, bus)
        } else {
            let mut bus = Bus::new(cartridge, model);
            bus.write_byte(0xFF40, 0x91);
            bus.write_byte(0xFF41, 0x81);

            match model {
                GBModel::DMG => {
                    Cpu::make_cpu(0x01B0, 0x0013, 0x00D8, 0x014D, 0x0100, 0xFFFE, model, bus)
                }
                GBModel::CGB => {
                    Cpu::make_cpu(0x1180, 0x0000, 0xFF56, 0x000D, 0x0100, 0xFFFE, model, bus)
                }
            }
        }
    }

    fn make_cpu(af: u16, bc: u16, de: u16, hl: u16, pc: u16, sp: u16, model: GBModel, bus: Bus) -> Self {
        Cpu { 
            bus,
            model,
            scheduled_ei: false,
            ime: false,
            halted: false,
            halt_bug: false,
            halt_triggered: false,
            t_cycles_so_far: 0,
            af: Register(af),
            bc: Register(bc),
            de: Register(de),
            hl: Register(hl),
            pc: Register(pc),
            sp: Register(sp),
            do_speed_switch: false,
        }
    }

    /// Steps through all parts of the emulator over the period
    /// that the next CPU instruction will take; returns that period's length in T-cycles.
    pub fn step(&mut self) -> u32 {
        let t_cycles = self.cycle();

        if matches!(self.model, GBModel::CGB) && self.do_speed_switch {
            self.do_speed_switch = false;
            // TODO: implement pausing after STOP instruction triggers speed switch
            return 2560 
        }

        self.bus.step(t_cycles);
       
        t_cycles
    }

    /// Do a CPU fetch-execute cycle and return the number of T-cycles taken.
    fn cycle(&mut self) -> u32 {
        self.halt_triggered = false;

        if self.scheduled_ei {
            self.ime = true;
            self.scheduled_ei = false;
        }
        
        let mut t_cycles = if !self.halted {
            self.execute_next_instruction() * 4
        } else {
            4
        };

        match self.get_pending_interrupt() {
            Some(interrupt) => {
                if self.ime {
                    t_cycles += self.handle_interrupt(interrupt) * 4;
                } else if self.halt_triggered {
                    self.halt_bug = true;
                }

                self.ime = false;
                self.halted = false;
            },
            None => {}
        }

        if t_cycles > self.t_cycles_so_far {
            self.bus.partial_step(t_cycles - self.t_cycles_so_far);
        }
        self.t_cycles_so_far = 0;

        t_cycles
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

    pub fn get_audio_output(&mut self) -> Option<[[f32; 2]; AUDIO_SAMPLES]> {
        self.bus.get_audio_output()
    }

    pub fn get_display_output(&mut self) -> Option<&[u8; LCD_BYTE_WIDTH * LCD_HEIGHT]> {
        self.bus.get_display_output()
    }

    pub fn entered_hblank(&self) -> bool {
        self.bus.entered_hblank()
    }

    pub fn update_joypad(&mut self, status: u8) {
        self.bus.update_joypad(status)
    }

    #[allow(dead_code)]
    pub fn get_serial_output(&self) -> &str {
        self.bus.get_serial_output()
    }

    pub fn save_mbc_state(&mut self) {
        self.bus.save_mbc_state()
    }

    #[allow(dead_code)]
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.bus.read_byte(addr)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn load_save(&mut self, data: Vec<u8>, save_type: &str) {
        self.bus.load_save(data, save_type);
    }

    #[cfg(target_arch = "wasm32")]
    pub fn save_id(&self) -> Option<String> {
        self.bus.save_id()
    }
}

#[cfg(test)]
mod tests {
    use super::test_helpers::test_blargg_rom;

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

#[cfg(test)]
pub mod test_helpers {
    use crate::Cartridge;
    use super::{Cpu, GBModel};

    const TEST_TIMEOUT: u64 = 1 << 32;

    fn mooneye_pass_check(cpu: &Cpu) -> bool {
        cpu.bc.hi() == 3 && 
        cpu.bc.lo() == 5 && 
        cpu.de.hi() == 8 && 
        cpu.de.lo() == 13 &&
        cpu.hl.hi() == 21 && 
        cpu.hl.lo() == 34
    }  
    
    fn mooneye_fail_check(cpu: &Cpu) -> bool {
        cpu.bc.hi() == 0x42 && 
        cpu.bc.lo() == 0x42 && 
        cpu.de.hi() == 0x42 && 
        cpu.de.lo() == 0x42 &&
        cpu.hl.hi() == 0x42 && 
        cpu.hl.lo() == 0x42
    }
    
    pub fn test_mooneye_rom(test_rom_path: &str, model: GBModel) {
        let cartridge = Cartridge::from_file(test_rom_path, false);
        let mut cpu = Cpu::new(cartridge, model);
    
        let mut cycles: u64 = 0;
        while cycles < TEST_TIMEOUT {
            cycles += cpu.step() as u64;
    
            if mooneye_fail_check(&cpu) {
                panic!("Mooneye Test Failed: {}", test_rom_path)
            } else if mooneye_pass_check(&cpu) {
                return;
            }
        } 
    }
    
    pub fn test_blargg_rom(test_rom_path: &str, model: GBModel) {
        let cartridge = Cartridge::from_file(test_rom_path, false);
        let mut cpu = Cpu::new(cartridge, model);
    
        let mut cycles: u64 = 0;
        while cycles < TEST_TIMEOUT {
            cycles += cpu.step() as u64;
    
            if cpu.get_serial_output().contains("Passed") {
                break;
            } else if cpu.get_serial_output().contains("Failed") {
                panic!("cpu_instr test ROM failed");
            }
        } 
    }
}