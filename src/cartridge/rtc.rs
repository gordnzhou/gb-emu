#[cfg(not(target_arch = "wasm32"))]
use std::time::SystemTime;

#[cfg(target_arch = "wasm32")]
use js_sys::Date;

pub const RTC_REGISTERS_SIZE: usize = 5;

# [derive(Clone)]
pub struct Rtc {
    out_registers: [u8; RTC_REGISTERS_SIZE],
    rtc_registers: [u8; RTC_REGISTERS_SIZE],
    register_time: u64,
    active_register: usize,
}

impl Rtc {
    /// Creates a brand new RTC register that counts starting from current time.
    pub fn new() -> Self {
        Rtc { 
            out_registers: [0; RTC_REGISTERS_SIZE],
            rtc_registers: [0; RTC_REGISTERS_SIZE],
            register_time: Rtc::get_current_time(),
            active_register: 0,
        }
    }

    /// Unparses current state of registers, along with register_time from save. 
    /// Updates rtc_registers with the time elapsed since the last save.
    pub fn from_save(save: [u8; RTC_REGISTERS_SIZE + 8]) -> Self {
        let mut registers = [0; RTC_REGISTERS_SIZE];
        let mut time_bytes = [0; 8];

        for i in 0..(RTC_REGISTERS_SIZE + 8) {
            if i < RTC_REGISTERS_SIZE {
                registers[i] = save[i];
            } else {
                time_bytes[i - RTC_REGISTERS_SIZE] = save[i];
            };
        }
        let register_time = u64::from_be_bytes(time_bytes);

        let mut rtc = Rtc {
            out_registers: [0; RTC_REGISTERS_SIZE],
            rtc_registers: registers,
            register_time,
            active_register: 0,
        };
        rtc.update_rtc_registers();
        rtc
    }

    /// Parses current state of registers, along with register_time to be saved.
    pub fn to_save(&self) -> [u8; RTC_REGISTERS_SIZE + 8] {
        let mut rtc = self.clone();
        rtc.update_rtc_registers();

        let mut save = [0; RTC_REGISTERS_SIZE + 8];
        let time_bytes = rtc.register_time.to_be_bytes();

        for i in 0..(RTC_REGISTERS_SIZE + 8) {
            save[i] = if i < RTC_REGISTERS_SIZE {
                rtc.rtc_registers[i]
            } else {
                time_bytes[i - RTC_REGISTERS_SIZE]
            };
        }

        save
    }

    pub fn set_active_reg(&mut self, byte: u8) {
        self.active_register = byte as usize - 8;
    }

    pub fn write(&mut self, byte: u8) {
        self.out_registers[self.active_register] = byte;
    }

    pub fn read(&self) -> u8 {
        self.out_registers[self.active_register]
    }

    /// Updates rtc clock data and latches it onto registers.
    pub fn latch_clock_data(&mut self) {
        self.update_rtc_registers();
        self.out_registers = self.rtc_registers
    }

    /// Updates current rtc_registers and sets register_time to the current time.
    fn update_rtc_registers(&mut self) {
        let current_time = Rtc::get_current_time();
        let elapsed =  current_time - self.register_time;

        self.rtc_registers[0] = ((self.rtc_registers[0] as u64 +  elapsed      )   % 60) as u8;
        self.rtc_registers[1] = ((self.rtc_registers[1] as u64 + (elapsed / 60))   % 60) as u8;
        self.rtc_registers[2] = ((self.rtc_registers[2] as u64 + (elapsed / 3600)) % 24) as u8;

        let extra_days = elapsed / (3600 * 24);
        let new_days = self.get_days() + extra_days;

        self.rtc_registers[3] = (new_days & 0xFF) as u8;
        self.rtc_registers[4] = self.rtc_registers[4] & 0xFE;
        if new_days >= 256 && new_days <= 511 {
            self.rtc_registers[4] |= 0x01;
        } else if new_days > 511 {
            self.rtc_registers[4] |= 0x80;
        }

        self.register_time = current_time;
    }

    fn get_days(&self) -> u64 {
        let days_lo = self.rtc_registers[3] as u64;
        let days_hi = self.rtc_registers[4] as u64 & 0x01;
        (days_hi << 8) | days_lo
    }

    /// Gets the current time represented as seconds elapsed since UNIX_EPOCH.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_current_time() -> u64 {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => {
                println!("Unable to get currrent system time");
                0
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_current_time() -> u64 {
        let date = Date::new_0();
        (date.get_time() / 1000.0) as u64
    }
}