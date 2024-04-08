use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Duration;

use gbemulib::constants::{LCD_BYTE_WIDTH, LCD_HEIGHT, LCD_WIDTH, T_CYCLE_DURATION_NS};
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::{AudioSubsystem, Sdl};
use sdl2::video::Window;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Canvas, Texture};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::cartridge::Cartridge;
use crate::cpu::{Cpu, GBModel};
use crate::config::{AUDIO_SAMPLES, SAMPLING_RATE_HZ};

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

pub const SCREEN_SCALE: i32 = 5;

pub const MASTER_VOLUME: f32 = 0.2;

const PIXEL_FORMAT: PixelFormatEnum = PixelFormatEnum::ARGB8888;

pub struct Emulator {
    event_pump: EventPump,
    canvas: Canvas<Window>,
    key_status: u8,
    cpu: Cpu,
    _audio_subsystem: AudioSubsystem,
    _audio_device: AudioDevice<Callback>,
    audio_tx: SyncSender<[[f32; 2]; AUDIO_SAMPLES]>
}

impl Emulator {
    /// Loads in given cartridge and initializes Gameboy emulator.
    pub fn load_cartridge(cartridge: Cartridge) -> Result<Self, String> {
        let sdl_context: Sdl = sdl2::init()?;

        let window_title = &cartridge.get_title();
        let canvas = Emulator::build_canvas(&sdl_context, SCREEN_SCALE as u32, window_title)?;
        let event_pump = sdl_context.event_pump()?;

        let (audio_tx, audio_rx) = std::sync::mpsc::sync_channel(4);
        let desired_spec = AudioSpecDesired {
            freq: Some(SAMPLING_RATE_HZ as i32),
            channels: Some(2),
            samples: Some(AUDIO_SAMPLES as u16),
        };
        let _audio_subsystem = sdl_context.audio()?;
        let _audio_device = _audio_subsystem.open_playback(None, &desired_spec, |_spec| {
            Callback { audio_rx, prev_sample: [0.0; 2] }
        }).unwrap();
        _audio_device.resume();

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
            cpu: Cpu::new(cartridge, model),
            _audio_device,
            _audio_subsystem,
            audio_tx,
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
            self.cpu.update_joypad(self.key_status);
            let t_cycles = self.cpu.step() as u64;
            self.step_emulator(&mut texture, rect);
            let cpu_duration_ns = t_cycles * T_CYCLE_DURATION_NS;
            dur_ns += cpu_duration_ns;
        } 
    }

    /// Steps SDL2 joypad input, texture display and audio callback
    fn step_emulator(&mut self, texture: &mut Texture, rect: Rect) {
        if self.cpu.entered_hblank() {
            match self.get_events() {
                Ok(_) => self.cpu.update_joypad(self.key_status),
                Err(e) => panic!("{}", e)
            }
        }

        match self.cpu.get_audio_output() {
            Some(audio_output) => self.audio_tx.send(audio_output).unwrap(),
            None => {}
        }

        match self.cpu.get_display_output() {
            Some(frame_buffer) => {
                texture
                    .update(None, frame_buffer, LCD_BYTE_WIDTH)
                    .expect("texture update failed");

                self.canvas.copy(&texture, None, rect).unwrap();
                self.canvas.present();
            }
            None => {}
        };
    }

    fn get_events(&mut self) -> Result<(), &str> { 
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.cpu.save_mbc_state();
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

struct Callback {
    audio_rx: Receiver<[[f32; 2]; AUDIO_SAMPLES]>,
    prev_sample: [f32; 2],
}

impl AudioCallback for Callback {
    type Channel = f32;

    fn callback(&mut self, stream: &mut [f32]) {
        match self.audio_rx.recv_timeout(Duration::from_millis(30)) {
            Ok(buffer) => {
                for i in 0..buffer.len() {
                    stream[i * 2] = buffer[i][0];
                    stream[i * 2 + 1] = buffer[i][1];
                }
            
                self.prev_sample = buffer[buffer.len() - 1];
            }
            Err(_) => {
                for i in 0..stream.len() {
                    stream[i] = self.prev_sample[i % 2];
                }
            }
        }

        for i in 0..stream.len() {
            stream[i] *= MASTER_VOLUME
        }
    }
}