use crate::joypad::Joypad;
use crate::apu::Apu;
use crate::ppu::Ppu;
use crate::timer::Timer;
use crate::memory::Memory;
use crate::cpu::Interrupt;

const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x0080;
// const TOTAL_SIZE: usize = 0x10000;

pub struct Bus {
    pub memory: Memory,
    pub joypad: Joypad,
    pub apu: Apu,
    pub ppu: Ppu,
    wram: [u8; WRAM_SIZE],
    timer: Timer, 
    hram: [u8; HRAM_SIZE],
    interrupt_enable: u8,
    interrupt_flag: u8,

    pub serial_output: String,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            memory: Memory::new(),
            joypad: Joypad::new(),
            apu: Apu::new(),
            ppu: Ppu::new(),
            timer: Timer::new(),
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            interrupt_enable: 0,
            interrupt_flag: 0xE0,

            serial_output: String::new(),
        }
    }

    /// Steps through APU, PPU and Timer, and updates interrupt flag(s).
    pub fn step(&mut self, cycles: u8) {
        self.ppu.step(cycles);
        self.apu.step(cycles);

        if self.ppu.entered_vblank {
            self.request_interrupt(Interrupt::VBlank);
        }
        if self.ppu.stat_triggered {
            self.request_interrupt(Interrupt::Stat)
        }
        if self.joypad.interrupt {
            self.request_interrupt(Interrupt::Joypad)
        }
        if self.timer.step(cycles) {
            self.request_interrupt(Interrupt::Timer)
        }
    }

    /// Returns byte from specified address; returns 0xFF for unused addresses.
    pub fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;

        match addr {
            0x0000..=0x7FFF => self.memory.read_rom(addr),
            0x8000..=0x9FFF => self.ppu.read_vram(addr),
            0xA000..=0xBFFF => self.memory.read_eram(addr),
            0xC000..=0xDFFF => self.wram[addr - 0xC000],
            0xE000..=0xFDFF => self.wram[addr - 0xE000],
            0xFE00..=0xFE9F => self.ppu.read_oam(addr),
            0xFEA0..=0xFEFF => 0xFF,

            // IO Registers
            0xFF00          => self.joypad.read_joypad(),
            0xFF04..=0xFF07 => self.timer.read_io(addr),
            0xFF0F          => self.interrupt_flag,
            0xFF10..=0xFF26 => self.apu.read_io(addr),
            0xFF30..=0xFF3F => self.apu.read_wave(addr),
            0xFF40..=0xFF4B => self.ppu.read_io(addr),
            0xFF50          => self.memory.read_bank(),
        
            0xFF80..=0xFFFE => self.hram[addr - 0xFF80],
            0xFFFF          => self.interrupt_enable,
            _               => 0xFF
        }
    }

    /// If specified address is writable, writes byte to it; MAY trigger an OAM DMA.
    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        let addr = addr as usize;

        match addr {
            0x0000..=0x7FFF => {},
            0x8000..=0x9FFF => self.ppu.write_vram(addr, byte),
            0xA000..=0xBFFF => {},
            0xC000..=0xDFFF => self.wram[addr - 0xC000] = byte,
            0xE000..=0xFDFF => self.wram[addr - 0xE000] = byte,
            0xFE00..=0xFE9F => self.ppu.write_oam(addr, byte),
            0xFEA0..=0xFEFF => {},

            // IO Registers
            0xFF00          => self.joypad.write_joypad(byte),
            0xFF01          => self.serial_output.push(char::from(byte)),
            0xFF04..=0xFF07 => self.timer.write_io(addr, byte),
            0xFF0F          => self.interrupt_flag = 0xE0 | byte,
            0xFF10..=0xFF26 => self.apu.write_io(addr, byte),
            0xFF30..=0xFF3F => self.apu.write_wave(addr, byte),
            0xFF40..=0xFF4B => self.ppu_write(addr, byte),
            0xFF50          => self.memory.write_bank(byte),

            0xFF80..=0xFFFE => self.hram[addr - 0xFF80] = byte,
            0xFFFF          => self.interrupt_enable = byte,
            _               => {},
        }
    }

    /// Returns big-endian word at \[addr\], \[addr + 1\].
    pub fn read_word(&self, addr: u16) -> u16 {
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    /// Writes word little-endian to \[addr\], \[addr + 1\].
    pub fn write_word(&mut self, addr: u16, word: u16) {
        self.write_byte(addr, word as u8);
        self.write_byte(addr.wrapping_add(1), (word >> 8) as u8);
    }

    fn ppu_write(&mut self, addr: usize, byte: u8) {
        let dma_write = self.ppu.write_io(addr, byte);
        if dma_write {
            self.transfer_to_oam((byte as u16) << 8);
        }
    }

    /// Starts a DMA transfer from 0xNN00-0xNN9F to 0xFE00-0xFE9F (OAM) 
    /// for 160 M-cycles (640 dots).
    fn transfer_to_oam(&mut self, start: u16) {
        for i in 0x00..=0x9F {
            let byte = self.read_byte(start | i);
            self.ppu.write_oam(0xFE00 | i as usize, byte);
        }
    }

    /// Sets given interrupt's bit in IF, which requests for that interrupt.
    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        
        match interrupt {
            Interrupt::VBlank => self.interrupt_flag |= 1 << 0,
            Interrupt::Stat   => self.interrupt_flag |= 1 << 1,
            Interrupt::Timer  => self.interrupt_flag |= 1 << 2,
            Interrupt::Serial => self.interrupt_flag |= 1 << 3,
            Interrupt::Joypad => self.interrupt_flag |= 1 << 4,
        }
    }
}