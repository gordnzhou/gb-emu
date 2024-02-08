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
            memory: [0; MEMORY_SIZE],
        }
    }

    // all 0xFFFF memory slots: read and write
    // AF, BC, DE, HL, PC, SP registers: read and write (includes flag registers)
    // call stack: push and pop
    // write-only IME flag (handles interrupts)
    // returns the number of clock M-cycles
    pub fn fetch_execute(&mut self) -> u8 {
        let opcode = self.memory[self.PC()];

        // executes instruction and returns number of clock cycles
        match opcode {
            0x00 => self.nop(),
            0x01 => self.ld_r16_n16("BC"),
            0x02 => self.ld_r16_a("BC"),
            0x03 => self.inc_r16("BC"),
            0x04 => self.inc_r8("B"),
            0x05 => self.dec_r8("B"),
            0x06 => self.ld_r8_n8("B"),
            0x07 => 0,
            0x08 => self.ld_n16_sp(),
            0x09 => self.add_hl_r16(self.BC()),
            0x0A => self.ld_a_r16("BC"),
            0x0B => self.dec_r16("BC"),
            0x0C => self.inc_r8("C"),
            0x0D => self.dec_r8("C"),
            0x0E => self.ld_r8_n8("C"),
            0x0F => 0,

            0x10 => self.stop(),
            0x11 => self.ld_r16_n16("DE"),
            0x12 => self.ld_r16_a("DE"),
            0x13 => self.inc_r16("DE"),
            0x14 => self.inc_r8("D"),
            0x15 => self.dec_r8("D"),
            0x16 => self.ld_r8_n8("D"),
            0x17 => 0,
            0x18 => self.jr_e8(),
            0x19 => self.add_hl_r16(self.DE()),
            0x1A => self.ld_a_r16("DE"),
            0x1B => self.dec_r16("DE"),
            0x1C => self.inc_r8("E"),
            0x1D => self.dec_r8("E"), 
            0x1E => self.ld_r8_n8("E"),
            0x1F => 0,

            0x20 => self.jr_cc_e8(self.cc_NZ()),
            0x21 => self.ld_r16_n16("HL"),
            0x22 => self.ld_hli_a(),
            0x23 => self.inc_r16("HL"),
            0x24 => self.inc_r8("H"),
            0x25 => self.dec_r8("H"),
            0x26 => self.ld_r8_n8("H"),
            0x27 => self.daa(),
            0x28 => self.jr_cc_e8(self.cc_Z()),
            0x29 => self.add_hl_r16(self.HL()),
            0x2A => self.ld_a_hli(),
            0x2B => self.dec_r16("HL"),
            0x2C => self.inc_r8("L"),
            0x2D => self.dec_r8("L"),
            0x2E => self.ld_r8_n8("L"),
            0x2F => self.cpl(),
            
            0x30 => self.jr_cc_e8(self.cc_NC()),
            0x31 => self.ld_r16_n16("SP"),
            0x32 => self.ld_hld_a(),
            0x33 => self.inc_r16("SP"),
            0x34 => self.inc_hl(),
            0x35 => self.dec_hl(),
            0x36 => self.ld_hl_n8(),
            0x37 => self.scf(),
            0x38 => self.jr_cc_e8(self.cc_C()),
            0x39 => self.add_hl_r16(self.SP()),
            0x3A => self.ld_a_hld(),
            0x3B => self.dec_r16("SP"),
            0x3C => self.inc_r8("A"),
            0x3D => self.dec_r8("A"),
            0x3E => self.ld_r8_n8("A"),
            0x3F => self.ccf(),

            0x40 => self.ld_r8_r8("B", "B"),
            0x41 => self.ld_r8_r8("B", "C"),
            0x42 => self.ld_r8_r8("B", "D"),
            0x43 => self.ld_r8_r8("B", "E"),
            0x44 => self.ld_r8_r8("B", "H"),
            0x45 => self.ld_r8_r8("B", "L"),
            0x46 => self.ld_r8_hl("B"),
            0x47 => self.ld_r8_r8("B", "A"),
            0x48 => self.ld_r8_r8("C", "B"),
            0x49 => self.ld_r8_r8("C", "C"),
            0x4A => self.ld_r8_r8("C", "D"),
            0x4B => self.ld_r8_r8("C", "E"),
            0x4C => self.ld_r8_r8("C", "H"),
            0x4D => self.ld_r8_r8("C", "L"),
            0x4E => self.ld_r8_hl("C"),
            0x4F => self.ld_r8_r8("C", "A"),

            0x50 => self.ld_r8_r8("D", "B"),
            0x51 => self.ld_r8_r8("D", "C"),
            0x52 => self.ld_r8_r8("D", "D"),
            0x53 => self.ld_r8_r8("D", "E"),
            0x54 => self.ld_r8_r8("D", "H"),
            0x55 => self.ld_r8_r8("D", "L"),
            0x56 => self.ld_r8_hl("D"),
            0x57 => self.ld_r8_r8("D", "A"),
            0x58 => self.ld_r8_r8("E", "B"),
            0x59 => self.ld_r8_r8("E", "C"),
            0x5A => self.ld_r8_r8("E", "D"),
            0x5B => self.ld_r8_r8("E", "E"),
            0x5C => self.ld_r8_r8("E", "H"),
            0x5D => self.ld_r8_r8("E", "L"),
            0x5E => self.ld_r8_hl("E"),
            0x5F => self.ld_r8_r8("E", "A"),

            0x60 => self.ld_r8_r8("H", "B"),
            0x61 => self.ld_r8_r8("H", "C"),
            0x62 => self.ld_r8_r8("H", "D"),
            0x63 => self.ld_r8_r8("H", "E"),
            0x64 => self.ld_r8_r8("H", "H"),
            0x65 => self.ld_r8_r8("H", "L"),
            0x66 => self.ld_r8_hl("H"),
            0x67 => self.ld_r8_r8("H", "A"),
            0x68 => self.ld_r8_r8("L", "B"),
            0x69 => self.ld_r8_r8("L", "C"),
            0x6A => self.ld_r8_r8("L", "D"),
            0x6B => self.ld_r8_r8("L", "E"),
            0x6C => self.ld_r8_r8("L", "H"),
            0x6D => self.ld_r8_r8("L", "L"),
            0x6E => self.ld_r8_hl("L"),
            0x6F => self.ld_r8_r8("L", "A"),

            0x70 => self.ld_hl_r8("B"),
            0x71 => self.ld_hl_r8("C"),
            0x72 => self.ld_hl_r8("D"),
            0x73 => self.ld_hl_r8("E"),
            0x74 => self.ld_hl_r8("H"),
            0x75 => self.ld_hl_r8("L"),
            0x76 => self.halt(),
            0x77 => self.ld_hl_r8("A"),
            0x78 => self.ld_r8_r8("A", "B"),
            0x79 => self.ld_r8_r8("A", "C"),
            0x7A => self.ld_r8_r8("A", "D"),
            0x7B => self.ld_r8_r8("A", "E"),
            0x7C => self.ld_r8_r8("A", "H"),
            0x7D => self.ld_r8_r8("A", "L"),
            0x7E => self.ld_r8_hl("A"),
            0x7F => self.ld_r8_r8("A", "A"),

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

            0x90 => self.sub_a_r8(self.B()),
            0x91 => self.sub_a_r8(self.C()),
            0x92 => self.sub_a_r8(self.D()),
            0x93 => self.sub_a_r8(self.E()),
            0x94 => self.sub_a_r8(self.H()),
            0x95 => self.sub_a_r8(self.L()),
            0x96 => self.sub_a_hl(),
            0x97 => self.sub_a_r8(self.A()),
            0x98 => self.sbc_a_r8(self.B()),
            0x99 => self.sbc_a_r8(self.C()),
            0x9A => self.sbc_a_r8(self.D()),
            0x9B => self.sbc_a_r8(self.E()),
            0x9C => self.sbc_a_r8(self.H()),
            0x9D => self.sbc_a_r8(self.L()),
            0x9E => self.sbc_a_hl(),
            0x9F => self.sbc_a_r8(self.A()),

            0xA0 => self.and_a_r8(self.B()),
            0xA1 => self.and_a_r8(self.C()),
            0xA2 => self.and_a_r8(self.D()),
            0xA3 => self.and_a_r8(self.E()),
            0xA4 => self.and_a_r8(self.H()),
            0xA5 => self.and_a_r8(self.L()),
            0xA6 => self.and_a_hl(),
            0xA7 => self.and_a_r8(self.A()),
            0xA8 => self.xor_a_r8(self.B()),
            0xA9 => self.xor_a_r8(self.C()),
            0xAA => self.xor_a_r8(self.D()),
            0xAB => self.xor_a_r8(self.E()),
            0xAC => self.xor_a_r8(self.H()),
            0xAD => self.xor_a_r8(self.L()),
            0xAE => self.xor_a_hl(),
            0xAF => self.xor_a_r8(self.A()),
            
            0xB0 => self.or_a_r8(self.B()),
            0xB1 => self.or_a_r8(self.C()),
            0xB2 => self.or_a_r8(self.D()),
            0xB3 => self.or_a_r8(self.E()),
            0xB4 => self.or_a_r8(self.H()),
            0xB5 => self.or_a_r8(self.L()),
            0xB6 => self.or_a_hl(),
            0xB7 => self.or_a_r8(self.A()),
            0xB8 => self.cp_a_r8(self.B()),
            0xB9 => self.cp_a_r8(self.C()),
            0xBA => self.cp_a_r8(self.D()),
            0xBB => self.cp_a_r8(self.E()),
            0xBC => self.cp_a_r8(self.H()),
            0xBD => self.cp_a_r8(self.L()),
            0xBE => self.cp_a_hl(),
            0xBF => self.cp_a_r8(self.A()),

            0xC0 => self.ret_cc(self.cc_NZ()),
            0xC1 => self.pop_r16("BC"),
            0xC2 => self.jp_cc_n16(self.cc_NZ()),
            0xC3 => self.jp_n16(),
            0xC4 => self.call_cc_n16(self.cc_NZ()),
            0xC5 => self.push_r16("BC"),
            0xC6 => self.add_a_n8(),
            0xC7 => self.rst(0x00),
            0xC8 => self.ret_cc(self.cc_Z()),
            0xC9 => self.ret(),
            0xCA => self.jp_cc_n16(self.cc_Z()),
            0xCB => self.cb_execute(),
            0xCC => self.call_cc_n16(self.cc_Z()),
            0xCD => self.call_n16(),
            0xCE => self.adc_a_n8(),
            0xCF => self.rst(0x08),

            0xD0 => self.ret_cc(self.cc_NC()),
            0xD1 => self.pop_r16("DE"),
            0xD2 => self.jp_cc_n16(self.cc_NC()),
            // 0xD3 => _,
            0xD4 => self.call_cc_n16(self.cc_NC()),
            0xD5 => self.push_r16("DE"),
            0xD6 => self.sub_a_n8(),
            0xD7 => self.rst(0x10),
            0xD8 => self.ret_cc(self.cc_C()),
            0xD9 => self.reti(),
            0xDA => self.jp_cc_n16(self.cc_C()),
            // 0xDB => _,
            0xDC => self.call_cc_n16(self.cc_C()),
            // 0xDD => _,
            0xDE => self.sbc_a_n8(),
            0xDF => self.rst(0x18),

            0xE0 => self.ldh_n16_a(),
            0xE1 => self.pop_r16("HL"),
            0xE2 => self.ldh_c_a(),
            // 0xE3 => _,
            // 0xE4 => _,
            0xE5 => self.push_r16("HL"),
            0xE6 => self.and_a_n8(),
            0xE7 => self.rst(0x20),
            0xE8 => self.add_sp_e8(),
            0xE9 => self.jp_hl(),
            0xEA => self.ld_n16_a(),
            // 0xEB => _,
            // 0xEC => _,
            // 0xED => _,
            0xEE => self.xor_a_n8(),
            0xEF => self.rst(0x28),
            
            0xF0 => self.ldh_a_n16(),
            0xF1 => self.pop_af(),
            0xF2 => self.ldh_a_c(),
            0xF3 => self.di(),
            // 0xF4 => _,
            0xF5 => self.push_af(),
            0xF6 => self.or_a_n8(),
            0xF7 => self.rst(0x30),
            0xF8 => self.ld_hl_sp_e8(),
            0xF9 => self.ld_sp_hl(),
            0xFA => self.ld_a_n16(),
            0xFB => self.ei(),
            // 0xFC => _,
            // 0xFD => _,
            0xFE => self.cp_a_n8(),
            0xFF => self.rst(0x38 ),
            _ => panic!("Unknown Opcode Encountered! {:#03x}", opcode),
        }
    }

    fn nop(&mut self) -> u8 {
        self.inc_PC(1);
        1
    }

    fn stop(&mut self) -> u8 {
        let _ = self.n8();
        self.inc_PC(2);
        2
    }

    // TODO
    fn di(&mut self) -> u8 {
        self.inc_PC(1);
        1
    }

    // TODO
    fn ei(&mut self) -> u8 {
        self.inc_PC(1);
        1
    }

    // TODO
    fn halt(&mut self) -> u8 {
        self.inc_PC(1);
        1
    }

    // TODO
    fn reti(&mut self) -> u8 {
        self.inc_PC(1);
        4
    }

    fn jp_n16(&mut self) -> u8 {
        self.set_PC(self.n16());
        4
    }

    fn jp_cc_n16(&mut self, cc : bool) -> u8 {
        if cc { self.jp_n16(); 4 }
        else { 3 }
    }

    fn jp_hl(&mut self) -> u8 {
        self.set_PC(self.HL());
        1
    }

    fn jr_e8(&mut self) -> u8 {
        let mut pc = self.PC() as i16;
        pc = pc.wrapping_add(self.e8() as i16);
        self.set_PC(pc as usize);
        3
    }

    fn jr_cc_e8(&mut self, cc: bool) -> u8 {
        if cc { self.jr_e8(); 3 }
        else { 2 }
    }

    fn rst(&mut self, vec: usize) -> u8 {
        self.push_stack(self.PC());
        self.set_PC(vec);
        4
    }

    fn call_n16(&mut self) -> u8 {
        self.push_stack(self.PC());
        self.set_PC(self.n16());
        6
    }

    fn call_cc_n16(&mut self, cc: bool) -> u8 {
        if cc { self.call_n16(); 6 }
        else { 3 }
    }

    fn ret(&mut self) -> u8 {
        let res = self.pop_stack();
        self.set_PC(res);
        4
    }

    fn ret_cc(&mut self, cc: bool) -> u8 {
        if cc { self.ret(); 5 }
        else { 2 }
    }

    fn pop_af(&mut self) -> u8 {
        let res = self.pop_stack();
        self.AF.set(res);
        self.inc_PC(1);
        3
    }

    fn pop_r16(&mut self, r16_name: &str) -> u8 {
        let res = self.pop_stack();
        self.set_r16(r16_name, res);
        self.inc_PC(1);
        3
    }

    fn push_af(&mut self) -> u8 {
        self.push_stack(self.AF());
        self.inc_PC(1);
        4
    }

    fn push_r16(&mut self, r16_name: &str) -> u8 {
        let r16 = self.r16(r16_name);
        self.push_stack(r16);
        self.inc_PC(1);
        4
    }

    fn pop_stack(&mut self) -> usize {
        let sp = self.SP();
        let lo = self.get_memory(sp);
        let hi = self.get_memory(sp.wrapping_add(1));
        self.set_SP(sp.wrapping_add(2));
        ((hi as usize) << 8) + lo as usize
    }

    fn push_stack(&mut self, val16: usize) {
        let hi = ((val16 & 0xFF00) >> 8) as u8;
        let lo = val16 as u8;
        let sp = self.SP();

        self.set_memory(sp.wrapping_sub(1), hi);
        self.set_memory(sp.wrapping_add(2), lo);
        self.set_SP(sp.wrapping_sub(2));
    }

    fn ld_r8_r8(&mut self, r8l_name: &str, r8r_name: &str) -> u8{
        self.set_r8(r8l_name, self.r8(r8r_name));
        self.inc_PC(1);
        1
    }

    fn ld_r8_n8(&mut self, r8_name: &str) -> u8 {
        self.set_r8(r8_name, self.n8());
        self.inc_PC(2);
        2
    }

    fn ld_r16_n16(&mut self, r16_name: &str) -> u8 {
        self.set_r16(r16_name, self.n16());
        self.inc_PC(3);
        3
    }

    fn ld_hl_r8(&mut self, r8_name: &str) -> u8{
        self.set_memory(self.HL(), self.r8(r8_name));
        self.inc_PC(1);
        2
    }

    fn ld_hl_n8(&mut self) -> u8{
        self.set_memory(self.HL(), self.n8());
        self.inc_PC(2);
        3
    }

    fn ld_r8_hl(&mut self, r8_name: &str) -> u8 {
        let hl = self.get_memory(self.HL());
        self.set_r8(r8_name, hl);
        self.inc_PC(1);
        2
    }

    fn ld_r16_a(&mut self, r16_name: &str) -> u8 {
        self.set_memory(self.r16(r16_name), self.A());
        self.inc_PC(1);
        2
    }

    fn ld_n16_a(&mut self) -> u8 {
        self.set_memory(self.n16(), self.A());
        self.inc_PC(3);
        4
    }

    fn ld_a_r16(&mut self, r16_name: &str) -> u8 {
        let r16 = self.get_memory(self.r16(r16_name));
        self.set_A(r16);
        self.inc_PC(1);
        2
    }

    fn ld_a_n16(&mut self) -> u8 {
        let n16 = self.get_memory(self.n16());
        self.set_A(n16);
        self.inc_PC(3);
        4
    }

    fn ld_hli_a(&mut self) -> u8 {
        let hl = self.HL();
        self.set_memory(hl, self.A());
        self.set_HL(hl.wrapping_add(1));
        self.inc_PC(1);
        2
    }

    fn ld_hld_a(&mut self) -> u8 {
        let hl = self.HL();
        self.set_memory(hl, self.A());
        self.set_HL(hl.wrapping_sub(1));
        self.inc_PC(1);
        2
    }

    fn ld_a_hli(&mut self) -> u8 {
        let hl = self.HL();
        self.set_A(self.get_memory(hl));
        self.set_HL(hl.wrapping_add(1));
        self.inc_PC(1);
        2
    }

    fn ld_a_hld(&mut self) -> u8 {
        let hl = self.HL();
        self.set_A(self.get_memory(hl));
        self.set_HL(hl.wrapping_sub(1));
        self.inc_PC(1);
        2
    }

    fn ld_n16_sp(&mut self) -> u8 {
        let sp = self.SP();
        self.set_memory(self.n16(), sp as u8);
        self.set_memory(self.n16().wrapping_add(1), (sp >> 8) as u8);
        self.inc_PC(3);
        5
    }

    fn ld_hl_sp_e8(&mut self) -> u8 {
        let sp = self.SP() as i32;
        let e8 = self.e8() as i32;
        let res = sp.wrapping_add(e8);
        // TODO
        self.set_all_flags(false, false, (sp ^ e8 ^ res) & 0x10 != 0, (sp ^ e8 ^ res) & 0x100 != 0);
        self.set_HL(res as usize);
        self.inc_PC(2);
        3
    }

    fn ld_sp_hl(&mut self) -> u8 {
        self.set_SP(self.HL());
        self.inc_PC(1);
        2
    }

    fn ldh_n16_a(&mut self) -> u8 {
        let n16 = 0xFF00 | self.n8() as usize;
        self.set_memory(n16, self.A());
        self.inc_PC(2);
        3
    }

    fn ldh_c_a(&mut self) -> u8 {
        let c = 0xFF00 | self.C() as usize;
        self.set_memory(c, self.A());
        self.inc_PC(1);
        2
    }

    fn ldh_a_n16(&mut self) -> u8 {
        let n16 = 0xFF00 | self.n8() as usize;
        self.set_A(self.get_memory(n16));
        self.inc_PC(2);
        3
    }

    fn ldh_a_c(&mut self) -> u8 {
        let c = 0xFF00 | self.C() as usize;
        self.set_A(self.get_memory(c));
        self.inc_PC(1);
        2
    }

    fn daa(&mut self) -> u8 {
        let a = self.A();
        let mut adjust = 0x00;
        if self.cflag() { adjust |= 0x60; };
        if self.hflag() { adjust |= 0x06; };
        
        let res =
            if self.nflag() {
                a.wrapping_sub(adjust)
            } else {
                if a > 0x99 { adjust |= 0x60; };
                if a & 0x0F > 0x09 { adjust |= 0x06; };
                a.wrapping_add(adjust)
            };
        
        self.set_A(res);
        self.set_all_flags(res == 0, self.nflag(), false, adjust & 0x60 != 0);
        self.inc_PC(1);
        1
    }

    fn cpl(&mut self) -> u8 {
        self.set_A(!self.A());
        self.set_all_flags(self.zflag(), true, true, self.cflag());
        self.inc_PC(1);
        1
    }

    fn scf(&mut self) -> u8 {
        self.set_all_flags(self.zflag(), false, false, true);
        self.inc_PC(1);
        1
    }

    fn ccf(&mut self) -> u8 {
        self.set_all_flags(self.zflag(), false, false, !self.cflag());
        self.inc_PC(1);
        1
    }

    fn dec_r8(&mut self, r8_name: &str) -> u8 {
        let r8 = self.r8(r8_name);
        let res = r8.wrapping_sub(1);
        self.set_all_flags(res == 0, true, ((r8 & 0xf) - 1) & 0x10 > 0, self.flag("C"));
        self.set_r8(r8_name, res);
        self.inc_PC(1);
        1
    }

    fn dec_hl(&mut self) -> u8 {
        let hl = self.get_memory(self.r16("HL"));
        let res = hl.wrapping_sub(1);
        self.set_all_flags(res == 0, true, ((hl & 0xf) - 1) & 0x10 > 0, self.flag("C"));
        self.set_memory(self.r16("HL"), res);
        self.inc_PC(1);
        3
    }

    fn dec_r16(&mut self, r16_name: &str) -> u8 {
        let r16 = self.r16(r16_name);
        self.set_r16(r16_name, r16.wrapping_add(1));
        self.inc_PC(1);
        2
    }

    fn inc_r8(&mut self, r8_name: &str) -> u8 {
        let r8 = self.r8(r8_name);
        let res = r8.wrapping_add(1);
        self.set_all_flags(res == 0, false, ((r8 & 0xF) + 1) & 0x10 > 0, self.flag("C"));
        self.set_r8(r8_name, res);
        self.inc_PC(1);
        1
    }

    fn inc_hl(&mut self) -> u8 {
        let hl = self.get_memory(self.r16("HL"));
        let res = hl.wrapping_add(1);
        self.set_all_flags(res == 0, false, ((hl & 0xF) + 1) & 0x10 > 0, self.flag("C"));
        self.set_memory(self.r16("HL"), res);
        self.inc_PC(1);
        3
    }

    fn inc_r16(&mut self, r16_name: &str) -> u8 {
        let r16 = self.r16(r16_name);
        self.set_r16(r16_name, r16.wrapping_add(1));
        self.inc_PC(1);
        2
    }

    fn xor_a_r8(&mut self, r8: u8) -> u8 {
        let res = self.A() ^ r8;
        self.set_A(res);
        self.set_all_flags(res == 0, false, false, false);
        self.inc_PC(1);
        1
    }

    fn xor_a_hl(&mut self) -> u8 {
        let res = self.A() ^ self.get_memory(self.HL());
        self.set_A(res);
        self.set_all_flags(res == 0, false, false, false);
        self.inc_PC(1);
        2
    }

    fn xor_a_n8(&mut self) -> u8 {
        let res = self.A() ^ self.n8();
        self.set_A(res);
        self.set_all_flags(res == 0, false, false, false);
        self.inc_PC(2);
        2
    }

    fn or_a_r8(&mut self, r8: u8) -> u8 {
        let res = self.A() | r8;
        self.set_A(res);
        self.set_all_flags(res == 0, false, false, false);
        self.inc_PC(1);
        1
    }

    fn or_a_hl(&mut self) -> u8 {
        let res = self.A() | self.get_memory(self.HL());
        self.set_A(res);
        self.set_all_flags(res == 0, false, false, false);
        self.inc_PC(1);
        2
    }

    fn or_a_n8(&mut self) -> u8 {
        let res = self.A() | self.n8();
        self.set_A(res);
        self.set_all_flags(res == 0, false, false, false);
        self.inc_PC(2);
        2
    }

    fn and_a_r8(&mut self, r8: u8) -> u8 {
        let res = self.A() & r8;
        self.set_A(res);
        self.set_all_flags(res == 0, false, true, false);
        self.inc_PC(1);
        1
    }

    fn and_a_hl(&mut self) -> u8 {
        let res = self.A() & self.get_memory(self.HL());
        self.set_A(res);
        self.set_all_flags(res == 0, false, true, false);
        self.inc_PC(1);
        2
    }

    fn and_a_n8(&mut self) -> u8 {
        let res = self.A() & self.n8();
        self.set_A(res);
        self.set_all_flags(res == 0, false, true, false);
        self.inc_PC(2);
        2
    }

    fn sbc_a_r8(&mut self, r8: u8) -> u8 {
        let r8 = r8 as u32;
        let res = self.sub_and_set_flags(self.A() as u32, r8, true);
        self.set_A(res);
        self.inc_PC(1);
        1
    }

    fn sbc_a_hl(&mut self) -> u8 {
        let hl = self.get_memory(self.HL()) as u32;
        let res = self.sub_and_set_flags(self.A() as u32, hl, true);
        self.set_A(res);
        self.inc_PC(1);
        2
    }

    fn sbc_a_n8(&mut self) -> u8 {
        let n8 = self.n8() as u32;
        let res = self.sub_and_set_flags(self.A() as u32, n8, true);
        self.set_A(res);
        self.inc_PC(2);
        2
    }

    fn sub_a_r8(&mut self, r8: u8) -> u8 {
        let r8 = r8 as u32;
        let res = self.sub_and_set_flags(self.A() as u32, r8, false);
        self.set_A(res);
        self.inc_PC(1);
        1
    }

    fn sub_a_hl(&mut self) -> u8 {
        let hl = self.get_memory(self.HL()) as u32;
        let res = self.sub_and_set_flags(self.A() as u32, hl, false);
        self.set_A(res);
        self.inc_PC(1);
        2
    }

    fn sub_a_n8(&mut self) -> u8 {
        let n8 = self.n8() as u32;
        let res = self.sub_and_set_flags(self.A() as u32, n8, false);
        self.set_A(res);
        self.inc_PC(2);
        2
    }

    fn cp_a_r8(&mut self, r8: u8) -> u8 {
        let r8 = r8 as u32;
        self.sub_and_set_flags(self.A() as u32, r8, false);
        self.inc_PC(1);
        1
    }

    fn cp_a_hl(&mut self) -> u8 {
        let hl = self.get_memory(self.HL()) as u32;
        self.sub_and_set_flags(self.A() as u32, hl, false);
        self.inc_PC(1);
        2
    }

    fn cp_a_n8(&mut self) -> u8 {
        let n8 = self.n8() as u32;
        self.sub_and_set_flags(self.A() as u32, n8, false);
        self.inc_PC(2);
        2
    }

    fn add_a_r8(&mut self, r8: u8) -> u8 {
        let r8 = r8 as u32;
        let res = self.add_and_set_flags(self.A() as u32, r8, false);
        self.set_A(res);
        self.inc_PC(1);
        1
    }

    fn add_a_hl(&mut self) -> u8 {
        let hl = self.get_memory(self.HL()) as u32;
        let res = self.add_and_set_flags(self.A() as u32, hl, false);
        self.set_A(res);
        self.inc_PC(1);
        2
    }

    fn add_a_n8(&mut self) -> u8 {
        let n8 = self.n8() as u32;
        let res = self.add_and_set_flags(self.A() as u32, n8, false);
        self.set_A(res);
        self.inc_PC(2);
        2
    }

    fn add_hl_r16(&mut self, r16: usize) -> u8 {
        let r16 = r16 as u32;
        let hl = self.get_memory(self.HL()) as u32;
        let res = self.add16_and_set_flags(hl, r16);
        self.set_HL(res);
        self.inc_PC(1);
        2
    }

    fn add_sp_e8(&mut self) -> u8 {
        let sp = self.SP() as i32;
        let e8 = self.e8() as i32;
        let res = sp.wrapping_add(e8);
        // TODO
        self.set_all_flags(false, false, (sp ^ e8 ^ res) & 0x10 != 0, (sp ^ e8 ^ res) & 0x100 != 0);
        self.set_SP(res as usize);
        self.inc_PC(2);
        4
    }

    fn adc_a_r8(&mut self, r8: u8) -> u8 {
        let r8 = r8 as u32;
        let res = self.add_and_set_flags(self.A() as u32, r8, true);
        self.set_A(res);
        self.inc_PC(1);
        1
    }

    fn adc_a_hl(&mut self) -> u8 {
        let hl = self.get_memory(self.HL()) as u32;
        let res = self.add_and_set_flags(self.A() as u32, hl, true);
        self.set_A(res);
        self.inc_PC(1);
        2
    }

    fn adc_a_n8(&mut self) -> u8 {
        let n8 = self.n8() as u32;
        let res = self.add_and_set_flags(self.A() as u32, n8 as u32, true);
        self.set_A(res);
        self.inc_PC(2);
        2
    }

    fn add16_and_set_flags(&mut self, a: u32, b: u32) -> usize {
        let r: u32 = a.wrapping_add(b);
        self.set_all_flags(self.zflag(), ((a & 0xFFF) + (b & 0xFFF)) & 0x1000 > 0, false, r > 0xFFFF);
        r as usize
    }

    fn sub_and_set_flags(&mut self, a: u32, b: u32, withCarry: bool) -> u8 {
        let carry: u32 = if withCarry { self.cflag() as u32 } else { 0 };
        let r: u32 = a.wrapping_sub(b).wrapping_sub(carry);
        self.set_all_flags(r == 0, true, ((a & 0xf) - (b & 0xf) - (carry & 0xf)) & 0x10 > 0, (b + carry) > a);
        r as u8
    }

    fn add_and_set_flags(&mut self, a: u32, b: u32, withCarry: bool) -> u8 {
        let carry: u32 = if withCarry { self.cflag() as u32 } else { 0 };
        let r: u32 = a.wrapping_add(b).wrapping_add(carry);
        self.set_all_flags(r == 0, false, ((a & 0xF) + (b & 0xF) + (carry & 0xF)) & 0x10 > 0, r > 0xFF);
        r as u8
    }

    fn cb_execute(&mut self, ) -> u8 {
        let opcode = self.memory[self.PC().wrapping_add(1)];

        match opcode {
            // replace all 1 answers with a '{}' like 0x00 and 0x01 and 0x02
            0x00 => {},
            0x01 => {},
            0x02 => {},
            0x03 => {},
            0x04 => {},
            0x05 => {},
            0x06 => {},
            0x07 => {},
            0x08 => {},
            0x09 => {},
            0x0A => {},
            0x0B => {},
            0x0C => {},
            0x0D => {},  
            0x0E => {},
            0x0F => {},

            0x10 => {},
            0x11 => {},
            0x12 => {},
            0x13 => {},
            0x14 => {},
            0x15 => {},
            0x16 => {},
            0x17 => {},
            0x18 => {},
            0x19 => {},
            0x1A => {},
            0x1B => {},
            0x1C => {},
            0x1D => {}, 
            0x1E => {},
            0x1F => {},

            0x20 => {},
            0x21 => {},
            0x22 => {},
            0x23 => {},
            0x24 => {},
            0x25 => {},
            0x26 => {},
            0x27 => {},
            0x28 => {},
            0x29 => {},
            0x2A => {},
            0x2B => {},
            0x2C => {},
            0x2D => {},
            0x2E => {},
            0x2F => {},
            
            0x30 => {},
            0x31 => {},
            0x32 => {},
            0x33 => {},
            0x34 => {},
            0x35 => {},
            0x36 => {},
            0x37 => {},
            0x38 => {},
            0x39 => {},
            0x3A => {},
            0x3B => {},
            0x3C => {},
            0x3D => {},
            0x3E => {},
            0x3F => {},

            0x40 => {},
            0x41 => {},
            0x42 => {},
            0x43 => {},
            0x44 => {},
            0x45 => {},
            0x46 => {},
            0x47 => {},
            0x48 => {},
            0x49 => {},
            0x4A => {},
            0x4B => {},
            0x4C => {},
            0x4D => {},
            0x4E => {},
            0x4F => {},

            0x50 => {},
            0x51 => {},
            0x52 => {},
            0x53 => {},
            0x54 => {},
            0x55 => {},
            0x56 => {},
            0x57 => {},
            0x58 => {},
            0x59 => {},
            0x5A => {},
            0x5B => {},
            0x5C => {},
            0x5D => {},
            0x5E => {},
            0x5F => {},

            0x60 => {},
            0x61 => {},
            0x62 => {},
            0x63 => {},
            0x64 => {},
            0x65 => {},
            0x66 => {},
            0x67 => {},
            0x68 => {},
            0x69 => {},
            0x6A => {},
            0x6B => {},
            0x6C => {},
            0x6D => {},
            0x6E => {},
            0x6F => {},

            0x70 => {},
            0x71 => {},
            0x72 => {},
            0x73 => {},
            0x74 => {},
            0x75 => {},
            0x76 => {},
            0x77 => {},
            0x78 => {},
            0x79 => {},
            0x7A => {},
            0x7B => {},
            0x7C => {},
            0x7D => {},
            0x7E => {},
            0x7F => {},

            0x80 => {},
            0x81 => {},
            0x82 => {},
            0x83 => {},
            0x84 => {},
            0x85 => {},
            0x86 => {},
            0x87 => {},
            0x88 => {},
            0x89 => {},
            0x8A => {},
            0x8B => {},
            0x8C => {},
            0x8D => {},
            0x8E => {},
            0x8F => {},

            0x90 => {},
            0x91 => {},
            0x92 => {},
            0x93 => {},
            0x94 => {},
            0x95 => {},
            0x96 => {},
            0x97 => {},
            0x98 => {},
            0x99 => {},
            0x9A => {},
            0x9B => {},
            0x9C => {},
            0x9D => {},
            0x9E => {},
            0x9F => {},

            0xA0 => {},
            0xA1 => {},
            0xA2 => {},
            0xA3 => {},
            0xA4 => {},
            0xA5 => {},
            0xA6 => {},
            0xA7 => {},
            0xA8 => {},
            0xA9 => {},
            0xAA => {},
            0xAB => {},
            0xAC => {},
            0xAD => {},
            0xAE => {},
            0xAF => {},
            
            0xB0 => {},
            0xB1 => {},
            0xB2 => {},
            0xB3 => {},
            0xB4 => {},
            0xB5 => {},
            0xB6 => {},
            0xB7 => {},
            0xB8 => {},
            0xB9 => {},
            0xBA => {},
            0xBB => {},
            0xBC => {},
            0xBD => {},
            0xBE => {},
            0xBF => {},

            0xC0 => {},
            0xC1 => {},
            0xC2 => {},
            0xC3 => {},
            0xC4 => {},
            0xC5 => {},
            0xC6 => {},
            0xC7 => {},
            0xC8 => {},
            0xC9 => {},
            0xCA => {},
            0xCB => {},
            0xCC => {},
            0xCD => {},
            0xCE => {},
            0xCF => {},

            0xD0 => {},
            0xD1 => {},
            0xD2 => {},
            0xD3 => {},
            0xD4 => {},
            0xD5 => {},
            0xD6 => {},
            0xD7 => {},
            0xD8 => {},
            0xD9 => {},
            0xDA => {},
            0xDB => {},
            0xDC => {},
            0xDD => {},
            0xDE => {},
            0xDF => {},

            0xE0 => {},
            0xE1 => {},
            0xE2 => {},
            0xE3 => {},
            0xE4 => {},
            0xE5 => {},
            0xE6 => {},
            0xE7 => {},
            0xE8 => {},
            0xE9 => {},
            0xEA => {},
            0xEB => {},
            0xEC => {},
            0xED => {},
            0xEE => {},
            0xEF => {},
            
            0xF0 => {},
            0xF1 => {},
            0xF2 => {},
            0xF3 => {},
            0xF4 => {},
            0xF5 => {},
            0xF6 => {},
            0xF7 => {},
            0xF8 => {},
            0xF9 => {},
            0xFA => {},
            0xFB => {},
            0xFC => {},
            0xFD => {},
            0xFE => {},
            0xFF => {},
        }

        self.inc_PC(2);
        if (opcode & 0xF)!= 0x6 && (opcode & 0xF) != 0xE { 2 }
        else { 
            if let 0x4..=0x7 = (opcode & 0xF0) >> 4 { 3 }
            else { 4 }
        }
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

    fn set_all_flags(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.set_zflag(z);
        self.set_nflag(n);
        self.set_hflag(h);
        self.set_cflag(c)
    }

    // Condition Codes
    fn cc_Z(&self) -> bool {  self.flag("Z") }
    fn cc_NZ(&self) -> bool { !self.flag("Z") }
    fn cc_C(&self) -> bool {  self.flag("C") }
    fn cc_NC(&self) -> bool { !self.flag("C") }

    // Register and Flag Getters
    fn r8(&self, name: &str) -> u8 {
        match name {
            "A" => { self.AF.hi() },
            "B" => { self.BC.hi() },
            "C" => { self.AF.lo() },
            "D" => { self.AF.hi() },
            "E" => { self.AF.lo() }
            "H" => { self.AF.hi() },
            "L" => { self.AF.lo() }
            _ => 0
        }
    }

    fn r16(&self, name: &str) -> usize {
        match name {
            "DE" => { self.DE.full() },
            "HL" => { self.HL.full() },
            "PC" => { self.PC.full() },
            "SP" => { self.SP.full() },
            _ => 0
        }
    }

    fn flag(&self, name: &str) -> bool {
        match name {
            "Z" => { self.AF.bit(7) },
            "N" => { self.AF.bit(6) },
            "H" => { self.AF.bit(5) },
            "C" => { self.AF.bit(4) },
            _ => false
        }
    }


    // Register and Flag Setters
    fn set_r8(&mut self, name: &str, val: u8) {
        match name {
            "A" => { self.AF.set_hi(val) },
            "B" => { self.BC.set_hi(val) },
            "C" => { self.AF.set_lo(val) },
            "D" => { self.AF.set_hi(val) },
            "E" => { self.AF.set_lo(val) }
            "H" => { self.AF.set_hi(val) },
            "L" => { self.AF.set_lo(val) }
            _ => {}
        }
    }

    fn set_r16(&mut self, name: &str, val: usize) {
        match name {
            "DE" => { self.DE.set(val) },
            "HL" => { self.HL.set(val) },
            "PC" => { self.PC.set(val) },
            "SP" => { self.SP.set(val) },
            _ => {}
        }
    }

    // Register and Flag Getters
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

    // Register and Flag Setters
    fn set_A(&mut self, val: u8) { self.AF.set_hi(val); }
    // fn set_B(&mut self, val: u8) { self.BC.set_hi(val); }
    // fn set_C(&mut self, val: u8) { self.BC.set_lo(val); }
    // fn set_D(&mut self, val: u8) { self.DE.set_hi(val); }
    // fn set_E(&mut self, val: u8) { self.DE.set_lo(val); }
    // fn set_H(&mut self, val: u8) { self.HL.set_hi(val); }
    // fn set_L(&mut self, val: u8) { self.HL.set_lo(val); }
    // fn set_BC(&mut self, val: usize) { self.BC.set(val); }
    // fn set_DE(&mut self, val: usize) { self.DE.set(val); }
    fn set_HL(&mut self, val: usize) { self.HL.set(val); }
    fn set_PC(&mut self, val: usize) { self.PC.set(val); }
    fn inc_PC(&mut self, val: usize) { self.PC.set(self.PC() + val); }
    fn set_SP(&mut self, val: usize) { self.SP.set(val); }
    fn set_cflag(&mut self, val: bool) { self.AF.set_bit(4, val); }
    fn set_hflag(&mut self, val: bool) { self.AF.set_bit(5, val); }
    fn set_nflag(&mut self, val: bool) { self.AF.set_bit(6, val); }
    fn set_zflag(&mut self, val: bool) { self.AF.set_bit(7, val); }
}