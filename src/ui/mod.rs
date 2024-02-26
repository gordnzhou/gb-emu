mod joypad;
mod apu;
mod ppu;

use sdl2::{Sdl, VideoSubsystem};
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use joypad::Joypad;
use apu::Apu;
use ppu::Ppu;

use crate::cpu::Interrupt;

const LCD_WIDTH: usize= 160;
const LCD_HEIGHT: usize = 144;

const COLOURS: [Color; 4] = [
    Color::RGB(155, 188, 15), // 00 -> White
    Color::RGB(139, 172, 15), // 01 -> Light Gray
    Color::RGB(48, 98, 48),   // 10 -> Dark Gray
    Color::RGB(15, 56, 15),   // 11 -> Black
];

pub struct Sdl2Wrapper {
    pub joypad: Joypad,
    pub apu: Apu,
    pub ppu: Ppu,

    event_pump: EventPump,
    screen_scale: i32,
    canvas: Canvas<Window>,
}

impl Sdl2Wrapper {
    pub fn new(screen_scale: i32) -> Result<Self, String> {
        let sdl_context: Sdl = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        // let audio_subsystem = sdl_context.audio()?;
        let event_pump = sdl_context.event_pump()?;
        let window = Sdl2Wrapper::build_window(video_subsystem, screen_scale as u32)?;
        let canvas: Canvas<Window> = window.into_canvas().build().map_err(|e| e.to_string())?;
        
        Ok(Sdl2Wrapper {
            joypad: Joypad::new(),
            apu: Apu::new(),
            ppu: Ppu::new(),
            event_pump,
            canvas,
            screen_scale,
        })
    }

    fn build_window(video_subsystem: VideoSubsystem, scale: u32) -> Result<Window, String> {
        let window_width = LCD_WIDTH as u32 * scale;
        let window_height = LCD_HEIGHT as u32 * scale;

        let window = video_subsystem
            .window("Gameboy Emulator", window_width, window_height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        println!("Created window of width {} and height {}", window_width, window_height);
        Ok(window)
    }

    fn get_events(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    panic!("Escape Entered");
                },
                _ => {}
            }
        }
    }

    /// Steps PPU, Display, Audio and Joypad Input over the given period (in cycles),
    /// returning any interrupts requested.
    pub fn step(&mut self, cycles: u8) -> Vec<Interrupt> {
        let mut interrupts = Vec::new();

        self.ppu.step(cycles);

        if self.ppu.entered_vblank {
            interrupts.push(Interrupt::VBlank);
            self.draw_window();
        }
        if self.ppu.stat_triggered {
            interrupts.push(Interrupt::Stat)
        }

        self.get_events();
        
        self.joypad.step();

        self.apu.step(cycles);

        interrupts
    }

    /// Renders frame buffer to SDL2 canvas (60 times per second).
    pub fn draw_window(&mut self) {
        let pixel_size = self.screen_scale as u32;

        for i in 0..LCD_HEIGHT {
            for j in 0..LCD_WIDTH {
                let x = j as i32 * self.screen_scale;
                let y = i as i32 * self.screen_scale;
                let colour = COLOURS[self.ppu.frame_buffer[i][j] as usize];

                self.canvas.set_draw_color(colour);
                let _ = self.canvas.fill_rect(Rect::new(x, y, pixel_size, pixel_size));
            }
        }

        self.canvas.present();
    }
}