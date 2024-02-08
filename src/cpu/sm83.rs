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
        let opcode = self.memory[self.r16("PC")];

        // executes instruction and returns number of clock cycles
        match opcode {
            // continue this pattern for all opcodes (0x00 to 0xFF)
            0x00 => 1,
            0x01 => self.ld_r16_n16("BC"),
            0x02 => self.ld_r16_a("BC"),
            0x03 => self.inc_r16("BC"),
            0x04 => self.inc_r8("B"),
            0x05 => self.dec_r8("B"),
            0x06 => self.ld_r8_n8("B"),
            0x07 => 1,
            0x08 => self.ld_n16_sp(),
            0x09 => self.add_hl_r16(self.BC()),
            0x0A => self.ld_a_r16("BC"),
            0x0B => self.dec_r16("BC"),
            0x0C => self.inc_r8("C"),
            0x0D => self.dec_r8("C"),
            0x0E => self.ld_r8_n8("C"),
            0x0F => 1,

            0x10 => 1,
            0x11 => self.ld_r16_n16("DE"),
            0x12 => self.ld_r16_a("DE"),
            0x13 => self.inc_r16("DE"),
            0x14 => self.inc_r8("D"),
            0x15 => self.dec_r8("D"),
            0x16 => self.ld_r8_n8("D"),
            0x17 => 1,
            0x18 => 1,
            0x19 => self.add_hl_r16(self.r16("DE")),
            0x1A => self.ld_a_r16("DE"),
            0x1B => self.dec_r16("DE"),
            0x1C => self.inc_r8("E"),
            0x1D => self.dec_r8("E"), 
            0x1E => self.ld_r8_n8("E"),
            0x1F => 1,

            0x20 => 1,
            0x21 => self.ld_r16_n16("HL"),
            0x22 => self.ld_hli_a(),
            0x23 => self.inc_r16("HL"),
            0x24 => self.inc_r8("H"),
            0x25 => self.dec_r8("H"),
            0x26 => self.ld_r8_n8("H"),
            0x27 => self.daa(),
            0x28 => 1,
            0x29 => self.add_hl_r16(self.HL()),
            0x2A => self.ld_a_hli(),
            0x2B => self.dec_r16("HL"),
            0x2C => self.inc_r8("L"),
            0x2D => self.dec_r8("L"),
            0x2E => self.ld_r8_n8("L"),
            0x2F => self.cpl(),
            
            0x30 => 1,
            0x31 => self.ld_r16_n16("SP"),
            0x32 => self.ld_hld_a(),
            0x33 => self.inc_r16("SP"),
            0x34 => self.inc_hl(),
            0x35 => self.dec_hl(),
            0x36 => self.ld_hl_n8(),
            0x37 => self.scf(),
            0x38 => 1,
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
            0x76 => 1, // HALT
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

            0xC0 => 1,
            0xC1 => self.pop_r16("BC"),
            0xC2 => 1,
            0xC3 => 1,
            0xC4 => 1,
            0xC5 => self.push_r16("BC"),
            0xC6 => self.add_a_n8(),
            0xC7 => 1,
            0xC8 => 1,
            0xC9 => 1,
            0xCA => 1,
            0xCB => 1, // CB OPCODES
            0xCC => 1,
            0xCD => 1,
            0xCE => self.adc_a_n8(),
            0xCF => 1,

            0xD0 => 1,
            0xD1 => self.pop_r16("DE"),
            0xD2 => 1,
            0xD3 => 1,
            0xD4 => 1,
            0xD5 => self.push_r16("DE"),
            0xD6 => self.sub_a_n8(),
            0xD7 => 1,
            0xD8 => 1,
            0xD9 => 1,
            0xDA => 1,
            0xDB => 1,
            0xDC => 1,
            0xDD => 1,
            0xDE => self.sbc_a_n8(),
            0xDF => 1,

            0xE0 => self.ldh_n16_a(),
            0xE1 => self.pop_r16("HL"),
            0xE2 => self.ldh_c_a(),
            0xE3 => 1,
            0xE4 => 1,
            0xE5 => self.push_r16("HL"),
            0xE6 => self.and_a_n8(),
            0xE7 => 1,
            0xE8 => self.add_sp_e8(),
            0xE9 => 1,
            0xEA => self.ld_n16_a(),
            0xEB => 1,
            0xEC => 1,
            0xED => 1,
            0xEE => self.xor_a_n8(),
            0xEF => 1,
            
            0xF0 => self.ldh_a_n16(),
            0xF1 => self.pop_af(),
            0xF2 => self.ldh_a_c(),
            0xF3 => 1,
            0xF4 => 1,
            0xF5 => self.push_af(),
            0xF6 => self.or_a_n8(),
            0xF7 => 1,
            0xF8 => self.ld_hl_sp_e8(),
            0xF9 => self.ld_sp_hl(),
            0xFA => self.ld_a_n16(),
            0xFB => 1,
            0xFC => 1,
            0xFD => 1,
            0xFE => self.cp_a_n8(),
            0xFF => 1,
        }
    }

    fn pop_af(&mut self) -> u8 {
        let res = self.pop_stack();
        self.AF.set_lo(res.0);
        self.set_A(res.1);
        self.inc_PC(1);
        3
    }

    fn pop_r16(&mut self, r16_name: &str) -> u8 {
        let res = self.pop_stack();
        self.set_r16(r16_name, (res.1 as usize >> 8) + res.0 as usize);
        self.inc_PC(1);
        3
    }

    fn pop_stack(&mut self) -> (u8, u8) {
        let sp = self.SP();
        let l = self.get_memory(sp);
        let r = self.get_memory(sp.wrapping_add(1));
        self.set_SP(sp.wrapping_add(2));
        (l, r)
    }

    fn push_af(&mut self) -> u8 {
        self.push_stack(self.AF.lo(), self.A());
        self.inc_PC(1);
        4
    }

    fn push_r16(&mut self, r16_name: &str) -> u8 {
        let r16 = self.r16(r16_name);
        self.push_stack((r16 & 0xFF00) as u8 >> 8, r16 as u8 & 0xFF);
        self.inc_PC(1);
        4
    }

    fn push_stack(&mut self, l: u8, r: u8) {
        let sp = self.SP();
        self.set_memory(sp.wrapping_sub(1), r);
        self.set_memory(sp.wrapping_add(2), l);
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

    fn set_flag(&mut self, name: &str, val: bool) {
        match name {
            "Z" => { self.AF.set_bit(7, val); },
            "N" => { self.AF.set_bit(6, val); },
            "H" => { self.AF.set_bit(5, val); },
            "C" => { self.AF.set_bit(4, val); },
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