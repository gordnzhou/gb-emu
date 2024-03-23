use sdl2::Sdl;

use crate::joypad::Joypad;
use crate::apu::Apu;
use crate::ppu::Ppu;
use crate::timer::Timer;
use crate::cartridge::Cartridge;
use crate::cpu::{GBModel, Interrupt};

const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x0080;

pub const ROM_START: usize = 0x0000;
pub const ROM_END: usize = 0x7FFF;
const VRAM_START: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
pub const RAM_START: usize = 0xA000;
pub const RAM_END: usize = 0xBFFF;
const WRAM_START: usize = 0xC000;
const WRAM_END: usize = 0xDFFF;
const WRAM2_START: usize = 0xE000;
const WRAM2_END: usize = 0xFDFF;
const OAM_START: usize = 0xFE00;
const OAM_END: usize = 0xFE9F;
const EMPTY_START: usize = 0xFEA0;
const EMPTY_END: usize = 0xFEFF;
const HRAM_START: usize = 0xFF80;
const HRAM_END: usize = 0xFFFE;

const DMA_CYCLES: u16 = 160;

pub struct Bus {
    model: GBModel,
    double_speed: bool,
    pub serial_output: String,

    pub cartridge: Cartridge,
    pub joypad: Joypad,
    pub apu: Apu,
    pub ppu: Ppu,
    wram: [u8; WRAM_SIZE],
    timer: Timer, 
    hram: [u8; HRAM_SIZE],
    interrupt_enable: u8,
    interrupt_flag: u8,
    dma_start: u16,
    dma_ticks: u16,

    // CGB ONLY
    key1: u8,
    hdma1: u8,
    hdma2: u8,
    hdma3: u8,
    hdma4: u8,
    hdma5: u8,
    rp: u8,
    svbk: u8,
}

impl Bus {
    pub fn new(cartridge: Cartridge, model: GBModel) -> Self {
        Bus {
            model,
            double_speed: false,
            serial_output: String::new(),

            cartridge,
            joypad: Joypad::new(),
            apu: Apu::new(None, model),
            ppu: Ppu::new(model),
            timer: Timer::new(),
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            interrupt_enable: 0,
            interrupt_flag: 0xE0,
            dma_start: 0,
            dma_ticks: DMA_CYCLES,

            key1: 0,
            hdma1: 0,
            hdma2: 0,
            hdma3: 0,
            hdma4: 0,
            hdma5: 0,
            rp: 0,
            svbk: 0,
        }
    }

    pub fn with_audio(mut self, sdl: Sdl) -> Self {
        self.apu = Apu::new(Some(sdl), self.model);
        self
    }

    /// Steps through components that require M-cycle level accuracy;
    /// this should also be called AFTER and BETWEEN (right after reads/writes) instructions.
    /// NOTE: This stepping is affected by double speed mode on CGB
    pub fn partial_step(&mut self, cycles: u8) {
        // step HDMA

        self.step_oam_dma(cycles);

        let old_div = self.timer.div;
        if self.timer.step(cycles) {
            self.request_interrupt(Interrupt::Timer)
        }
        
        if self.double_speed {
            if old_div & 0x20 != 0 && self.timer.div & 0x20 == 0 {
                self.apu.frame_sequencer_step();
            }
        } else {
            if old_div & 0x10 != 0 && self.timer.div & 0x10 == 0 {
                self.apu.frame_sequencer_step();
            }
        }
    }

    /// Steps through other components to be done at the END OF EACH INTSTRUCTION.
    /// Updates interrupt flags accordingly.
    pub fn step(&mut self, cycles: u8) {
        self.apu.step(cycles as u32);
        
        self.ppu.step(cycles);

        if self.ppu.entered_vblank {
            self.request_interrupt(Interrupt::VBlank);
        }
        if self.ppu.stat_triggered {
            self.request_interrupt(Interrupt::Stat)
        }
        if self.joypad.interrupt {
            self.request_interrupt(Interrupt::Joypad)
        }
    }

    /// Returns byte from specified address; returns 0xFF for unused addresses.
    pub fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;

        match addr {
            ROM_START..=ROM_END     => self.cartridge.read_rom(addr),
            VRAM_START..=VRAM_END   => self.ppu.read_vram(addr),
            RAM_START..=RAM_END     => self.cartridge.read_ram(addr),
            WRAM_START..=WRAM_END   => self.wram[addr - WRAM_START],
            WRAM2_START..=WRAM2_END => self.wram[addr - WRAM2_START],
            OAM_START..=OAM_END     => self.ppu.read_oam(addr),
            EMPTY_START..=EMPTY_END => 0xFF,

            // IO Registers
            0xFF00          => self.joypad.read_joypad(),
            0xFF04..=0xFF07 => self.timer.read_io(addr),
            0xFF0F          => self.interrupt_flag,
            0xFF10..=0xFF26 => self.apu.read_io(addr),
            0xFF30..=0xFF3F => self.apu.read_io(addr),
            0xFF40..=0xFF4B => self.ppu.read_io(addr),
            0xFF50          => self.cartridge.read_bank(),

            // CGB Registers
            0xFF4D if self.is_cgb() => self.key1, 
            0xFF4F if self.is_cgb() => self.ppu.read_io(addr),
            0xFF55 if self.is_cgb() => self.hdma5,
            0xFF56 if self.is_cgb() => self.rp,
            0xFF68..=0xFF6C if self.is_cgb() => self.ppu.read_io(addr),
            0xFF70 if self.is_cgb() => self.svbk,
            0xFF76 if self.is_cgb() => self.apu.read_io(addr),
            0xFF77 if self.is_cgb() => self.apu.read_io(addr),
        
            HRAM_START..=HRAM_END => self.hram[addr - HRAM_START],
            0xFFFF          => self.interrupt_enable,
            _               => 0xFF
        }
    }

    /// If specified address is writable, writes byte to it; MAY trigger an OAM DMA.
    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        let addr = addr as usize;

        match addr {
            ROM_START..=ROM_END     => self.cartridge.write_rom(addr, byte),
            VRAM_START..=VRAM_END   => self.ppu.write_vram(addr, byte),
            RAM_START..=RAM_END     => self.cartridge.write_ram(addr, byte),
            WRAM_START..=WRAM_END   => self.wram[addr - WRAM_START] = byte,
            WRAM2_START..=WRAM2_END => self.wram[addr - WRAM2_START] = byte,
            OAM_START..=OAM_END     => self.ppu.write_oam(addr, byte),
            EMPTY_START..=EMPTY_END => {},

            // IO Registers
            0xFF00          => self.joypad.write_joypad(byte),
            0xFF01          => self.serial_output.push(char::from(byte)),
            0xFF04..=0xFF07 => self.timer.write_io(addr, byte),
            0xFF0F          => self.interrupt_flag = 0xE0 | byte,
            0xFF10..=0xFF26 => self.apu.write_io(addr, byte),
            0xFF30..=0xFF3F => self.apu.write_io(addr, byte),
            0xFF46          => self.write_dma(byte),
            0xFF40..=0xFF4B => self.ppu.write_io(addr, byte),
            0xFF50          => self.cartridge.write_bank(byte),

            // CGB Registers
            0xFF4D if self.is_cgb() => self.key1 = byte & 0x7F,
            0xFF4F if self.is_cgb() => self.ppu.write_io(addr, byte),
            0xFF51 if self.is_cgb() => self.hdma1 = byte,
            0xFF52 if self.is_cgb() => self.hdma2 = byte,
            0xFF53 if self.is_cgb() => self.hdma3 = byte,
            0xFF54 if self.is_cgb() => self.hdma4 = byte,
            0xFF55 if self.is_cgb() => self.hdma5 = byte,
            0xFF56 if self.is_cgb() => self.rp = byte & 0xFD,
            0xFF68..=0xFF6C if self.is_cgb() => self.ppu.write_io(addr, byte),
            0xFF70 if self.is_cgb() => self.svbk = byte,

            HRAM_START..=HRAM_END => self.hram[addr - HRAM_START] = byte,
            0xFFFF          => self.interrupt_enable = byte,
            _               => {},
        }
    }

    /// Writes to DMA register and initializes an OAM DMA transfer.
    fn write_dma(&mut self, byte: u8) {
        self.ppu.write_dma(byte);
        self.dma_start = (byte as u16) << 8;
        self.dma_ticks = 0;
        self.step_oam_dma(1);
    }

    /// Steps through a DMA transfer from 0xNN00-0xNN9F to 0xFE00-0xFE9F (OAM) 
    /// which runs for 160 M-cycles in total.
    fn step_oam_dma(&mut self, cycles: u8) {
        let mut cycles = cycles;
        while cycles > 0 && self.dma_ticks < DMA_CYCLES {

            // One byte transferred per M cycle during OAM DMA.\
            let dma_index = self.dma_ticks;
            let byte = self.read_byte(self.dma_start | dma_index);
            self.ppu.write_oam(0xFE00 | dma_index as usize, byte);

            cycles -=1;
            self.dma_ticks += 1;
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

    /// If speed switch has been armed, unarms it, switches speed and returns true.
    pub fn speed_switch(&mut self) -> bool {
        if self.is_cgb() && self.key1 & 1 != 0 {
            self.key1 &= 0xFE;
            self.key1 = !(self.key1 & 0x80) | (self.key1 & 0x7F);
            self.double_speed = !self.double_speed;
            self.timer.div = 0;
            return true;
        } 
        false
    }

    fn is_cgb(&self) -> bool {
        matches!(self.model, GBModel::CGB)
    }
}