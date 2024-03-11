use sdl2::{Sdl, VideoSubsystem};
use sdl2::video::Window;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use sdl2::audio::{AudioQueue, AudioSpecDesired};

use crate::cpu::Cpu;

// in order of: START, SELECT, B, A, DOWN, UP, LEFT, RIGHT.
pub const KEYMAPPINGS: [Keycode; 8] = [
    Keycode::I,
    Keycode::J,
    Keycode::K,
    Keycode::L,
    Keycode::S,
    Keycode::W,
    Keycode::A,
    Keycode::D,
];

pub const LCD_WIDTH: usize= 160;
pub const LCD_HEIGHT: usize = 144;
pub const BYTES_PER_PIXEL: usize = 4;
pub const LCD_BYTE_WIDTH: usize = BYTES_PER_PIXEL * LCD_WIDTH;

pub const DOT_HZ: u32 = 1 << 22;
pub const CYCLE_HZ: u32 = DOT_HZ >> 2;

// 1 dot = 2^22 Hz = 1/4 M-cycle = 238.4... ns
pub const DOT_DURATION_NS: f32 = 1e9 / DOT_HZ as f32;
const CYCLE_DURATION_NS: f32 = DOT_DURATION_NS * 4.0;

pub const SAMPLING_RATE_HZ: u32 = 48000;
pub const AUDIO_SAMPLES: usize = 2048;

use std::time::{Duration, Instant};

pub struct Emulator {
    event_pump: EventPump,
    screen_scale: i32,
    canvas: Canvas<Window>,
    audio_device: AudioQueue<f32>,
    key_status: u8,
    cpu: Cpu,
}

impl Emulator {
    pub fn new(screen_scale: i32, rom_path: &str, skip_bootrom: bool) -> Result<Self, String> {
        let sdl_context: Sdl = sdl2::init()?;

        let video_subsystem = sdl_context.video()?;
        let window = Emulator::build_window(video_subsystem, screen_scale as u32)?;
        let canvas: Canvas<Window> = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;

        let desired_spec = AudioSpecDesired {
            freq: Some(SAMPLING_RATE_HZ as i32),
            channels: Some(2),
            samples: Some(AUDIO_SAMPLES as u16 ),
        };

        let audio_subsystem = sdl_context.audio()?;
        let audio_device = audio_subsystem.open_queue(None, &desired_spec)?;
        audio_device.resume();

        let mut cpu = if !skip_bootrom {
            let mut cpu = Cpu::new(0, 0, 0, 0, 0, 0);
            cpu.bus.memory.load_bootrom();
            cpu
        } else {
            Cpu::new(0x01B0, 0x0013, 0x00D8, 0x014D, 0x0100, 0xFFFE)
        };

        cpu.bus.memory.load_from_file(rom_path);

        Ok(Emulator {
            event_pump,
            canvas,
            screen_scale,
            audio_device,
            key_status: 0xFF,
            cpu,
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

    /// Runs the emulator.
    // pub fn run(&mut self) {
    //     let mut last_instr = Instant::now();
    //     let mut cpu_duration_ns: f32 = 0.0;

    //     loop {
    //         if last_instr.elapsed() >= Duration::from_nanos(cpu_duration_ns as u64) {
    //             last_instr = Instant::now();
    //             let cycles = self.cpu.step();

    //             cpu_duration_ns = cycles as f32 * (DOT_DURATION_NS * 4.0);

    //             self.play_audio();
    //             // self.draw_window();
    //         }

    //         match self.get_events() {
    //             Ok(_) => self.cpu.bus.joypad.step(self.key_status),
    //             Err(e) => panic!("{}", e)
    //         }
    //     }
    // }

    /// Runs the emulator for the specified number of nanoseconds.
    pub fn debug_run(&mut self, total_dur_ns: u64) {
        let mut dur_ns = 0;

        let mut last_instr = Instant::now();
        let mut cpu_duration_ns: u64 = 0;

        let creator = self.canvas.texture_creator();
        let mut texture = creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, LCD_WIDTH as u32, LCD_HEIGHT as u32)
            .map_err(|e| e.to_string())
            .unwrap();

        let screen_width = LCD_WIDTH as u32 * self.screen_scale as u32;
        let screen_height = LCD_HEIGHT as u32 * self.screen_scale as u32;
        let rect = Rect::new(0, 0, screen_width, screen_height);

        while dur_ns < total_dur_ns {
            if last_instr.elapsed() >= Duration::from_nanos(cpu_duration_ns) {
                last_instr = Instant::now();
                let cycles = self.cpu.step();
                self.cpu.bus.joypad.step(self.key_status);
                self.play_audio();
                self.next_frame(&mut texture, rect);

                cpu_duration_ns = (cycles as f32 * CYCLE_DURATION_NS) as u64;
                dur_ns += cpu_duration_ns;
            }
        } 
    }

    fn play_audio(&mut self) {
        if self.cpu.bus.apu.buffer_index >= AUDIO_SAMPLES {
            self.audio_device.queue_audio(&self.cpu.bus.apu.audio_buffer).unwrap();
            self.cpu.bus.apu.buffer_index = 0;
        }
    }

    fn get_events(&mut self) -> Result<(), &str> { 
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Err("User Exited");
                },
                Event::KeyDown { keycode: Some(key), ..} => {   
                    for i in 0..8 {
                        if KEYMAPPINGS[i] == key {
                            self.key_status &= !(1 << (7 - i));
                        }
                    }
                }
                Event::KeyUp { keycode: Some(key), .. } => {
                    for i in 0..8 {
                        if KEYMAPPINGS[i] == key {
                            self.key_status |= 1 << (7 - i);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Renders frame buffer to SDL2 canvas (60 times per second).
    fn next_frame(&mut self, texture: &mut Texture, rect: Rect) {
        if !self.cpu.bus.ppu.entered_vblank {
            return;
        }

        match self.get_events() {
            Ok(_) => self.cpu.bus.joypad.step(self.key_status),
            Err(e) => panic!("{}", e)
        }

        texture
            .update(None, &self.cpu.bus.ppu.frame_buffer, LCD_BYTE_WIDTH)
            .expect("texture update failed");

        self.canvas.copy(&texture, None, rect).unwrap();
        self.canvas.present();
    }
}