use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::cartridge::Cartridge;
use crate::cpu::{Cpu, GBModel};
use crate::{KEYMAPPINGS, SCREEN_SCALE};

pub const LCD_WIDTH: usize= 160;
pub const LCD_HEIGHT: usize = 144;

const PIXEL_FORMAT: PixelFormatEnum = PixelFormatEnum::ARGB8888;
pub const BYTES_PER_PIXEL: usize = 4;

pub const LCD_BYTE_WIDTH: usize = BYTES_PER_PIXEL * LCD_WIDTH;

// DETERMINES GAME SPEED
pub const DOT_HZ: u32 = 1 << 22;

pub const CYCLE_HZ: u32 = DOT_HZ >> 2;

// 1 dot = 2^22 Hz = 1/4 M-cycle = 238.4... ns
pub const DOT_DURATION_NS: f32 = 1e9 / DOT_HZ as f32;
const CYCLE_DURATION_NS: f32 = DOT_DURATION_NS * 4.0;

pub struct Emulator {
    event_pump: EventPump,
    canvas: Canvas<Window>,
    key_status: u8,
    cpu: Cpu,
}

impl Emulator {
    /// Loads in given cartridge and initializes Gameboy emulator.
    pub fn load_cartridge(cartridge: Cartridge) -> Result<Self, String> {
        let sdl_context: Sdl = sdl2::init()?;

        let window_title = &cartridge.get_title();
        let canvas = Emulator::build_canvas(&sdl_context, SCREEN_SCALE as u32, window_title)?;
        let event_pump = sdl_context.event_pump()?;

        let model = if cartridge.cgb_compatible() {
            GBModel::CGB
        } else {
            GBModel::DMG
        };
        println!("detected model: {:?}", model);

        Ok(Emulator {
            event_pump,
            canvas,
            key_status: 0xFF,
            cpu: Cpu::new(cartridge, model).with_audio(sdl_context),
        })
    }

    fn build_canvas(sdl_context: &Sdl, scale: u32, title: &str) -> Result<Canvas<Window>, String> {
        let video_subsystem = sdl_context.video()?;
        let window_width = LCD_WIDTH as u32 * scale;
        let window_height = LCD_HEIGHT as u32 * scale;

        let window = video_subsystem
            .window("Gameboy Emulator", window_width, window_height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        println!("Created window of width {} and height {}", window_width, window_height);

        let mut canvas: Canvas<Window> = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        let title = &format!("MelonBoy | Playing: {}", title);
        canvas.window_mut().set_title(title).unwrap();
        Ok(canvas)
    }

    /// Runs the emulator for the specified number of nanoseconds.
    pub fn run_for_duration(&mut self, total_dur_ns: u64) {
        let mut dur_ns = 0;

        let creator = self.canvas.texture_creator();
        let mut texture = creator
            .create_texture_streaming(PIXEL_FORMAT, LCD_WIDTH as u32, LCD_HEIGHT as u32)
            .map_err(|e| e.to_string())
            .unwrap();

        let screen_width = LCD_WIDTH as u32 * SCREEN_SCALE as u32;
        let screen_height = LCD_HEIGHT as u32 * SCREEN_SCALE as u32;
        let rect = Rect::new(0, 0, screen_width, screen_height);

        // NOTE: cycle timings seem to be controlled by APU audio callback 
        while dur_ns < total_dur_ns {
            self.cpu.bus.joypad.step(self.key_status);
            let cycles = self.cpu.step();
            self.next_frame(&mut texture, rect);

            let cpu_duration_ns = (cycles as f32 * CYCLE_DURATION_NS) as u64;
            dur_ns += cpu_duration_ns;
        } 
    }

    /// Gets user input and updates screen, once every frame.
    fn next_frame(&mut self, texture: &mut Texture, rect: Rect) {
        if self.cpu.bus.ppu.entered_hblank() {
            match self.get_events() {
                Ok(_) => self.cpu.bus.joypad.step(self.key_status),
                Err(e) => panic!("{}", e)
            }
        }
        
        if !self.cpu.bus.ppu.entered_vblank {
            return;
        }
        self.cpu.bus.ppu.entered_vblank = false;

        texture
            .update(None, &self.cpu.bus.ppu.frame_buffer, LCD_BYTE_WIDTH)
            .expect("texture update failed");

        self.canvas.copy(&texture, None, rect).unwrap();
        self.canvas.present();
    }

    fn get_events(&mut self) -> Result<(), &str> { 
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.cpu.bus.cartridge.save_mbc_state();
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
}