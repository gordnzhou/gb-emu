use crate::cpu::Cpu;

// 1 dot = 2^22 Hz = 1/4 M-cycle = 238.4 ns
pub const DOT_DURATION_NS: f32 = 1e9 / (1 << 22) as f32;

use std::time::{Duration, Instant};

pub struct Emulator {
    cpu: Cpu
}

impl Emulator {
    pub fn new(cpu: Cpu) -> Self{
        Emulator {
            cpu
        }
    }

    /// Runs the emulator.
    #[allow(dead_code)]
    pub fn run(&mut self) {
        let mut last_instr = Instant::now();
        let mut cpu_duration_ns: f32 = 0.0;

        loop {
            if last_instr.elapsed() >= Duration::from_nanos(cpu_duration_ns as u64) {
                last_instr = Instant::now();
                let cycles = self.cpu.step();

                cpu_duration_ns = 4.0 * cycles as f32 * DOT_DURATION_NS;
            }
        }
    }

    /// Runs the emulator for the specified number of nanoseconds.
    pub fn debug_run(&mut self, total_dur_ns: u64) {
        let mut dur_ns = 0;

        let mut last_instr = Instant::now();
        let mut cpu_duration_ns: u64 = 0;

        while dur_ns < total_dur_ns {
            if last_instr.elapsed() >= Duration::from_nanos(cpu_duration_ns) {
                last_instr = Instant::now();

                let cycles = self.cpu.step();

                cpu_duration_ns = (4.0 * cycles as f32 * DOT_DURATION_NS) as u64;

                dur_ns += cpu_duration_ns;
            }
        } 
    }
}