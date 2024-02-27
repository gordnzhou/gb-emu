use crate::ui::Sdl2Wrapper;
use crate::timer::Timer;
use crate::rom::Rom;
use crate::cpu::Interrupt;

use std::fs::File;
use std::io::{self, Read};

const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x0080;
const TOTAL_SIZE: usize = 0x10000;

pub struct Mmu {
    rom: Rom,
    pub(super) sdl2_wrapper: Sdl2Wrapper, 
    wram: [u8; WRAM_SIZE],
    timer: Timer, 
    hram: [u8; HRAM_SIZE],
    interrupt_enable: u8,
    interrupt_flag: u8,

    // for unused addresses
    total_memory: [u8; TOTAL_SIZE],
    old_tma: u8,
}

impl Mmu {
    pub fn new(sdl2_wrapper: Sdl2Wrapper) -> Self {
        Mmu {
            rom: Rom::new(),
            sdl2_wrapper,
            timer: Timer::new(),
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            interrupt_enable: 0,
            interrupt_flag: 0,

            total_memory: [0; TOTAL_SIZE],
            old_tma: 0,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;

        match addr {
            0x0000..=0x7FFF => self.rom.read_rom(addr),
            0x8000..=0x9FFF => self.sdl2_wrapper.ppu.read_vram(addr),
            0xA000..=0xBFFF => self.rom.read_eram(addr),
            0xC000..=0xDFFF => self.wram[addr - 0xC000],
            0xE000..=0xFDFF => self.wram[addr - 0xE000],
            0xFE00..=0xFE9F => self.sdl2_wrapper.ppu.read_oam(addr),
            0xFEA0..=0xFEFF => 0xFF, // TODO: return 0x00 during OAM block trigger for dmg
            0xFF00 => self.sdl2_wrapper.joypad.read_joypad(),
            0xFF04 => self.timer.read_div(),
            0xFF05 => self.timer.read_tima(),
            0xFF06 => self.timer.read_tma(),
            0xFF07 => self.timer.read_tac(),
            0xFF0F => self.interrupt_flag,

            0xFF40 => self.sdl2_wrapper.ppu.read_lcdc(),
            0xFF41 => self.sdl2_wrapper.ppu.read_stat(),
            0xFF42 => self.sdl2_wrapper.ppu.read_scy(),
            0xFF43 => self.sdl2_wrapper.ppu.read_scx(),
            0xFF44 => self.sdl2_wrapper.ppu.read_ly(),
            0xFF45 => self.sdl2_wrapper.ppu.read_lyc(),
            0xFF46 => self.sdl2_wrapper.ppu.read_dma(),
            0xFF47 => self.sdl2_wrapper.ppu.read_bgp(),
            0xFF48 => self.sdl2_wrapper.ppu.read_obp0(),
            0xFF49 => self.sdl2_wrapper.ppu.read_obp1(),
            0xFF4A => self.sdl2_wrapper.ppu.read_wy(),
            0xFF4B => self.sdl2_wrapper.ppu.read_wx(),
            // IO registers from 0xFF4D to 0xFF77 are have special uses only in CGB
            0xFF80..=0xFFFE => self.hram[addr - 0xFF80],
            0xFFFF => self.interrupt_enable,
            _ => self.total_memory[addr]
        }
    }

    pub fn read_word(&self, addr: u16) -> u16{
        let lo = self.read_byte(addr) as u16;
        let hi = self.read_byte(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        let addr = addr as usize;

        match addr {
            0x0000..=0x7FFF => self.rom.write_rom(addr, byte),
            0x8000..=0x9FFF => self.sdl2_wrapper.ppu.write_vram(addr, byte),
            0xA000..=0xBFFF => self.rom.write_eram(addr, byte),
            0xC000..=0xDFFF => self.wram[addr - 0xC000] = byte,
            0xE000..=0xFDFF => self.wram[addr - 0xE000] = byte,
            0xFE00..=0xFE9F => self.sdl2_wrapper.ppu.write_oam(addr, byte),
            0xFEA0..=0xFEFF => {},
            0xFF00 => self.sdl2_wrapper.joypad.write_joypad(byte),
            0xFF04 => self.timer.write_div(byte),
            0xFF05 => self.timer.write_tima(byte),
            0xFF06 => { self.old_tma = self.timer.read_tima(); self.timer.write_tma(byte)},
            0xFF07 => self.timer.write_tac(byte),
            0xFF0F => self.interrupt_flag = byte,
            
            0xFF40 => self.sdl2_wrapper.ppu.write_lcdc(byte),
            0xFF41 => self.sdl2_wrapper.ppu.write_stat(byte),
            0xFF42 => self.sdl2_wrapper.ppu.write_scy(byte),
            0xFF43 => self.sdl2_wrapper.ppu.write_scx(byte),
            0xFF44 => {},
            0xFF45 => self.sdl2_wrapper.ppu.write_lyc(byte),
            0xFF46 => { 
                self.sdl2_wrapper.ppu.write_dma(byte); 
                // self.transfer_to_oam(byte as u16 >> 8);
            },
            0xFF47 => self.sdl2_wrapper.ppu.write_bgp(byte),
            0xFF48 => self.sdl2_wrapper.ppu.write_obp0(byte),
            0xFF49 => self.sdl2_wrapper.ppu.write_obp1(byte),
            0xFF4A => self.sdl2_wrapper.ppu.write_wy(byte),
            0xFF4B => self.sdl2_wrapper.ppu.write_wx(byte),
            // IO registers from 0xFF4D to 0xFF77 are have special uses only in CGB
            0xFF80..=0xFFFE => self.hram[addr - 0xFF80] = byte,
            0xFFFF => self.interrupt_enable = byte,
            _ => self.total_memory[addr] = byte,
        }
    }

    pub fn write_word(&mut self, addr: u16, word: u16) {
        self.write_byte(addr, word as u8);
        self.write_byte(addr.wrapping_add(1), (word >> 8) as u8);
    }

    /// TODO: Starts a DMA transfer from 0xNN00-0xNN9F to 0xFE00-0xFE9F (OAM) 
    /// after for 160 M-cycles (640 dots). Disables read/writes to 
    /// all of memory, except for HRAM.
    #[allow(dead_code)]
    pub fn transfer_to_oam(&mut self, start: u16) {
        // self.dma_locked = 160;
        for i in 0x00..=0x9F {
            let byte = self.read_byte(start | i);
            self.write_byte(0xFE00 | i, byte);
        }
    }

    /// Steps through components, updating interrupt flag.
    // TODO: refactor interrupt representation to be updated as struct fields for each respective component
    pub fn step(&mut self, cycles: u8) {
        let interrupts = self.sdl2_wrapper.step(cycles);

        self.step_timer(cycles);

        for interrupt in interrupts {
            self.request_interrupt(interrupt)
        }
    }

    fn step_timer(&mut self, cycles: u8) {
        // old TMA is used by timer in case TMA is written on the same cycle where TIMA is set to TMA
        let cur_tma = self.timer.read_tma();
        self.timer.write_tma(self.old_tma);

        if self.timer.step(cycles) {
            self.request_interrupt(Interrupt::Timer)
        }

        self.timer.write_tma(cur_tma)
    }

    /// Sets given interrupt's bit in IF, effectively requesting for the interrupt.
    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        
        match interrupt {
            Interrupt::VBlank => self.interrupt_flag |= 1 << 0,
            Interrupt::Stat   => self.interrupt_flag |= 1 << 1,
            Interrupt::Timer  => self.interrupt_flag |= 1 << 2,
            Interrupt::Serial => self.interrupt_flag |= 1 << 3,
            Interrupt::Joypad => self.interrupt_flag |= 1 << 4,
        }
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        match Mmu::read_rom_from_file(rom_path) {
            Ok(rom_data) => {
                for i in 0..rom_data.len() {
                    self.write_byte(i as u16, rom_data[i]);
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

    fn read_rom_from_file(file_path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(file_path)?;

        let mut rom_data = Vec::new();
        file.read_to_end(&mut rom_data)?;

        Ok(rom_data)
    }
}