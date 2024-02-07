#![allow(non_snake_case)]
use crate::register::Register;

const MEMORY_SIZE: usize = 0xFFFF;

pub struct SM83 {
    AF: Register,
    BC: Register,
    DE: Register,
    HL: Register,
    PC: Register,
    SP: Register,
    stack: Vec<usize>,
    memory: [u8; MEMORY_SIZE],
}

impl SM83 {
    pub fn new() -> Self {
        SM83 { 
            AF: Register(0),
            BC: Register(0),
            DE: Register(0),
            HL: Register(0),
            PC: Register(0x100),
            SP: Register(0),
            stack: Vec::new(),
            memory: [0; MEMORY_SIZE],
        }
    }

    // all 0xFFFF memory slots: read and write
    // AF, BC, DE, HL, PC, SP registers: read and write (includes flag registers)
    // call stack: push and pop
    // write-only IME flag (handles interrupts)
    // returns the number of clock M-cycles
    pub fn fetch_execute(&mut self) -> u8 {
        let opcode = self.memory[self.PC.full()];

        // executes instruction and returns number of clock cycles
        match opcode {
            // continue this pattern for all opcodes (0x00 to 0xFF)
            0x00 => 1,
            0x01 => 1,
            0x02 => 1,
            0x03 => 1,
            0x04 => 1,
            0x05 => 1,
            0x06 => 1,
            0x07 => 1,
            0x08 => 1,
            0x09 => self.add_hl_r16(self.BC()),
            0x0A => 1,
            0x0B => 1,
            0x0C => 1,
            0x0D => 1,  
            0x0E => 1,
            0x0F => 1,

            0x10 => 1,
            0x11 => 1,
            0x12 => 1,
            0x13 => 1,
            0x14 => 1,
            0x15 => 1,
            0x16 => 1,
            0x17 => 1,
            0x18 => 1,
            0x19 => self.add_hl_r16(self.DE()),
            0x1A => 1,
            0x1B => 1,
            0x1C => 1,
            0x1D => 1, 
            0x1E => 1,
            0x1F => 1,

            0x20 => 1,
            0x21 => 1,
            0x22 => 1,
            0x23 => 1,
            0x24 => 1,
            0x25 => 1,
            0x26 => 1,
            0x27 => 1,
            0x28 => 1,
            0x29 => self.add_hl_r16(self.HL()),
            0x2A => 1,
            0x2B => 1,
            0x2C => 1,
            0x2D => 1,
            0x2E => 1,
            0x2F => 1,
            
            0x30 => 1,
            0x31 => 1,
            0x32 => 1,
            0x33 => 1,
            0x34 => 1,
            0x35 => 1,
            0x36 => 1,
            0x37 => 1,
            0x38 => 1,
            0x39 => self.add_hl_r16(self.SP()),
            0x3A => 1,
            0x3B => 1,
            0x3C => 1,
            0x3D => 1,
            0x3E => 1,
            0x3F => 1,

            0x40 => 1,
            0x41 => 1,
            0x42 => 1,
            0x43 => 1,
            0x44 => 1,
            0x45 => 1,
            0x46 => 1,
            0x47 => 1,
            0x48 => 1,
            0x49 => 1,
            0x4A => 1,
            0x4B => 1,
            0x4C => 1,
            0x4D => 1,
            0x4E => 1,
            0x4F => 1,

            0x50 => 1,
            0x51 => 1,
            0x52 => 1,
            0x53 => 1,
            0x54 => 1,
            0x55 => 1,
            0x56 => 1,
            0x57 => 1,
            0x58 => 1,
            0x59 => 1,
            0x5A => 1,
            0x5B => 1,
            0x5C => 1,
            0x5D => 1,
            0x5E => 1,
            0x5F => 1,

            0x60 => 1,
            0x61 => 1,
            0x62 => 1,
            0x63 => 1,
            0x64 => 1,
            0x65 => 1,
            0x66 => 1,
            0x67 => 1,
            0x68 => 1,
            0x69 => 1,
            0x6A => 1,
            0x6B => 1,
            0x6C => 1,
            0x6D => 1,
            0x6E => 1,
            0x6F => 1,

            0x70 => 1,
            0x71 => 1,
            0x72 => 1,
            0x73 => 1,
            0x74 => 1,
            0x75 => 1,
            0x76 => 1,
            0x77 => 1,
            0x78 => 1,
            0x79 => 1,
            0x7A => 1,
            0x7B => 1,
            0x7C => 1,
            0x7D => 1,
            0x7E => 1,
            0x7F => 1,

            0x80 => self.add_a_r8(self.B()),
            0x81 => self.add_a_r8(self.C()),
            0x82 => self.add_a_r8(self.D()),
            0x83 => self.add_a_r8(self.E()),
            0x84 => self.add_a_r8(self.H()),
            0x85 => self.add_a_r8(self.L()),
            0x86 => self.add_a_hl(),
            0x87 => self.add_a_r8(self.A()),
            0x88 => self.adc_a_r8(self.B()),
            0x89 => self.adc_a_r8(self.C()),
            0x8A => self.adc_a_r8(self.D()),
            0x8B => self.adc_a_r8(self.E()),
            0x8C => self.adc_a_r8(self.H()),
            0x8D => self.adc_a_r8(self.L()),
            0x8E => self.adc_a_hl(),
            0x8F => self.adc_a_r8(self.A()),

            0x90 => 1,
            0x91 => 1,
            0x92 => 1,
            0x93 => 1,
            0x94 => 1,
            0x95 => 1,
            0x96 => 1,
            0x97 => 1,
            0x98 => 1,
            0x99 => 1,
            0x9A => 1,
            0x9B => 1,
            0x9C => 1,
            0x9D => 1,
            0x9E => 1,
            0x9F => 1,

            0xA0 => 1,
            0xA1 => 1,
            0xA2 => 1,
            0xA3 => 1,
            0xA4 => 1,
            0xA5 => 1,
            0xA6 => 1,
            0xA7 => 1,
            0xA8 => 1,
            0xA9 => 1,
            0xAA => 1,
            0xAB => 1,
            0xAC => 1,
            0xAD => 1,
            0xAE => 1,
            0xAF => 1,
            
            0xB0 => 1,
            0xB1 => 1,
            0xB2 => 1,
            0xB3 => 1,
            0xB4 => 1,
            0xB5 => 1,
            0xB6 => 1,
            0xB7 => 1,
            0xB8 => 1,
            0xB9 => 1,
            0xBA => 1,
            0xBB => 1,
            0xBC => 1,
            0xBD => 1,
            0xBE => 1,
            0xBF => 1,

            0xC0 => 1,
            0xC1 => 1,
            0xC2 => 1,
            0xC3 => 1,
            0xC4 => 1,
            0xC5 => 1,
            0xC6 => self.add_a_n8(),
            0xC7 => 1,
            0xC8 => 1,
            0xC9 => 1,
            0xCA => 1,
            0xCB => 1,
            0xCC => 1,
            0xCD => 1,
            0xCE => self.adc_a_n8(),
            0xCF => 1,

            0xD0 => 1,
            0xD1 => 1,
            0xD2 => 1,
            0xD3 => 1,
            0xD4 => 1,
            0xD5 => 1,
            0xD6 => 1,
            0xD7 => 1,
            0xD8 => 1,
            0xD9 => 1,
            0xDA => 1,
            0xDB => 1,
            0xDC => 1,
            0xDD => 1,
            0xDE => 1,
            0xDF => 1,

            0xE0 => 1,
            0xE1 => 1,
            0xE2 => 1,
            0xE3 => 1,
            0xE4 => 1,
            0xE5 => 1,
            0xE6 => 1,
            0xE7 => 1,
            0xE8 => self.add_sp_e8(),
            0xE9 => 1,
            0xEA => 1,
            0xEB => 1,
            0xEC => 1,
            0xED => 1,
            0xEE => 1,
            0xEF => 1,
            
            0xF0 => 1,
            0xF1 => 1,
            0xF2 => 1,
            0xF3 => 1,
            0xF4 => 1,
            0xF5 => 1,
            0xF6 => 1,
            0xF7 => 1,
            0xF8 => 1,
            0xF9 => 1,
            0xFA => 1,
            0xFB => 1,
            0xFC => 1,
            0xFD => 1,
            0xFE => 1,
            0xFF => 1,
            _ => 0
        }
    }

    fn add_a_r8(&mut self, r8: u8) -> u8 {
        let r8 = r8 as u32;
        let res = self.add_and_set_flags(self.A() as u32, r8);
        self.set_A(res);
        self.inc_PC(1);
        1
    }

    fn add_a_hl(&mut self) -> u8 {
        let hl = self.get_memory(self.HL()) as u32;
        let res = self.add_and_set_flags(self.A() as u32, hl);
        self.set_A(res);
        self.inc_PC(1);
        2
    }

    fn add_a_n8(&mut self) -> u8 {
        let n8 = self.n8() as u32;
        let res = self.add_and_set_flags(self.AF.hi() as u32, n8);
        self.set_A(res);
        self.inc_PC(2);
        2
    }

    fn add_hl_r16(&mut self, r16: usize) -> u8 {
        let r16 = r16 as u32;
        let res = self.add16_and_set_flags(self.HL() as u32, r16);
        self.set_HL(res);
        self.inc_PC(1);
        2
    }

    // TODO: 
    // fn ld_hl_sp_e8(&mut self) -> u8 {
    //     self.add_sp_e8();
    //     self.set_HL(self.SP());
    //     3
    // }

    fn add_sp_e8(&mut self) -> u8 {
        let sp = self.SP() as i32;
        let e8 = self.e8() as i32;
        let res = sp.wrapping_add(e8);
        self.set_zflag(false);
        self.set_nflag(false);
        self.set_hflag((sp ^ e8 ^ res) & 0x10 != 0);
        self.set_cflag((sp ^ e8 ^ res) & 0x100 != 0);
        self.set_SP(res as usize);
        self.inc_PC(2);
        4
    }

    fn add16_and_set_flags(&mut self, a: u32, b: u32) -> usize {
        let r: u32 = a.wrapping_add(b);
        self.set_nflag(false);
        self.set_hflag((a ^ b ^ r) & 0x1000 != 0);
        self.set_cflag(r & 0x10000 != 0);
        r as usize
    }

    fn adc_a_r8(&mut self, r8: u8) -> u8 {
        let r8 = r8 as u32;
        let r8_plus_carry = r8.wrapping_add(self.cflag() as u32);
        let res = self.add_and_set_flags(self.A() as u32, r8_plus_carry);
        self.set_A(res);
        self.inc_PC(1);
        1
    }

    fn adc_a_hl(&mut self) -> u8 {
        let hl = self.get_memory(self.HL()) as u32;
        let hl_plus_carry = hl.wrapping_add(self.cflag() as u32);
        let res = self.add_and_set_flags(self.AF.hi() as u32, hl_plus_carry);
        self.set_A(res);
        self.inc_PC(1);
        2
    }

    fn adc_a_n8(&mut self) -> u8 {
        let n8 = self.n8() as u32;
        let n8_plus_carry = n8.wrapping_add(self.cflag() as u32);
        let res = self.add_and_set_flags(self.AF.hi() as u32, n8_plus_carry);
        self.set_A(res);
        self.inc_PC(2);
        2
    }

    fn add_and_set_flags(&mut self, a: u32, b: u32) -> u8 {
        let r: u32 = a.wrapping_add(b);
        
        self.set_zflag(r == 0);
        self.set_nflag(false);
        self.set_hflag((a ^ b ^ r) & 0x10 != 0);
        self.set_cflag(r & 0x100 != 0);
        r as u8
    }


    fn n8(&self) -> u8 {
        self.get_memory(self.PC().wrapping_add(1))
    }

    fn n16(&self) -> usize {
        self.get_memory(self.PC().wrapping_add(1)) as usize + 
        ((self.get_memory(self.PC().wrapping_add(2)) as usize) << 8)
    }

    fn e8(&self) -> i8 {
        self.n8() as i8
    }

    fn get_memory(&self, address: usize) -> u8 {
        self.memory[address]
    }

    fn set_memory(&mut self, address: usize, val: u8) {
        self.memory[address] = val;
    }

    fn A(&self) -> u8 { self.AF.hi() }
    fn B(&self) -> u8 { self.BC.hi() }
    fn C(&self) -> u8 { self.BC.lo() }
    fn D(&self) -> u8 { self.DE.hi() }
    fn E(&self) -> u8 { self.DE.lo() }
    fn H(&self) -> u8 { self.HL.hi() }
    fn L(&self) -> u8 { self.HL.lo() }
    fn AF(&self) -> usize { self.AF.full() }
    fn BC(&self) -> usize { self.BC.full() }
    fn DE(&self) -> usize { self.DE.full() }
    fn HL(&self) -> usize { self.HL.full() }
    fn PC(&self) -> usize { self.PC.full() }
    fn SP(&self) -> usize { self.SP.full() }
    fn cflag(&self) -> bool { self.AF.bit(4) }
    fn hflag(&self) -> bool { self.AF.bit(5) }
    fn nflag(&self) -> bool { self.AF.bit(6) }
    fn zflag(&self) -> bool { self.AF.bit(7) }

    fn set_A(&mut self, val: u8) { self.AF.set_hi(val); }
    fn set_B(&mut self, val: u8) { self.BC.set_hi(val); }
    fn set_C(&mut self, val: u8) { self.BC.set_lo(val); }
    fn set_D(&mut self, val: u8) { self.DE.set_hi(val); }
    fn set_E(&mut self, val: u8) { self.DE.set_lo(val); }
    fn set_H(&mut self, val: u8) { self.HL.set_hi(val); }
    fn set_L(&mut self, val: u8) { self.HL.set_lo(val); }
    fn set_BC(&mut self, val: usize) { self.BC.set(val); }
    fn set_DE(&mut self, val: usize) { self.DE.set(val); }
    fn set_HL(&mut self, val: usize) { self.HL.set(val); }
    fn inc_PC(&mut self, val: usize) { self.PC.set(self.PC() + val); }
    fn set_SP(&mut self, val: usize) { self.SP.set(val); }
    fn set_cflag(&mut self, val: bool) { self.AF.set_bit(4, val); }
    fn set_hflag(&mut self, val: bool) { self.AF.set_bit(5, val); }
    fn set_nflag(&mut self, val: bool) { self.AF.set_bit(6, val); }
    fn set_zflag(&mut self, val: bool) { self.AF.set_bit(7, val); }
}