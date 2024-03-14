use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

const HEADER_SIZE: usize = 0x50;
const HEADER_START: usize = 0x100;

const LOGO_BYTES: usize = 0x30;

pub struct Header {
    pub nintendo_logo: [u8; LOGO_BYTES],
    pub manufacturer_code: String,
    pub cgb_flag: u8,
    pub licensee_code: u16,
    pub sgb_flag: u8,
    pub destination_code: u8, 
    pub version_number: u8,
    pub header_checksum: u8, 
    pub global_checksum: u16,

    title: String,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8, 
}

impl Header {
    /// Constructs cartridge header using bytes from addresses 0x100 to 0x14F.
    pub fn from_file(rom_path: &str) -> io::Result<Self> {
        let mut file = File::open(rom_path)?;
        file.seek(SeekFrom::Start(HEADER_START as u64))?;

        let mut header_bytes = vec![0; HEADER_SIZE];
        file.read_exact(&mut header_bytes)?;

        let nintendo_logo = header_bytes[0x04..=0x33]
            .try_into()
            .unwrap();
        
        let title_bytes = header_bytes[0x34..=0x43].to_vec();
        let title = match String::from_utf8(title_bytes) {
            Ok(s) => s,
            Err(_) => String::from("")
        };

        let manufacturer_code = match String::from_utf8(header_bytes[0x3F..=0x42].to_vec()) {
            Ok(s) => s,
            Err(_) => String::from("")
        };

        let cgb_flag = header_bytes[0x43];

        let old_licensee_code = header_bytes[0x4B];
        let licensee_code = if old_licensee_code != 0x33 {
            old_licensee_code as u16
        } else {
            ((header_bytes[0x44] as u16) << 8) | header_bytes[0x45] as u16
        };

        let sgb_flag = header_bytes[0x46];

        let cartridge_type = header_bytes[0x47];

        let rom_size = header_bytes[0x48];

        let ram_size = header_bytes[0x49];

        let version_number = header_bytes[0x4C];

        let header_checksum = header_bytes[0x4D];

        // unused for most games
        let global_checksum = ((header_bytes[0x4E] as u16) << 8) | header_bytes[0x4F] as u16;

        let destination_code = header_bytes[0x4A];

        let mut checksum: u8 = 0;
        for i in 0x34..=0x4C {
            checksum = checksum.wrapping_sub(header_bytes[i]).wrapping_sub(1);
        }

        assert!(checksum == header_checksum, "Invalid Header Checksum");

        Ok(Header {
            nintendo_logo,
            title,
            manufacturer_code,
            cgb_flag,
            licensee_code,
            sgb_flag,
            cartridge_type,
            rom_size,
            ram_size, 
            destination_code, 
            version_number,
            header_checksum, 
            global_checksum,
        })
    }

    pub fn num_rom_banks(&self) -> usize {
        match self.rom_size {
            0x00 => 2,
            0x01 => 4, // Unused
            0x02 => 8,
            0x03 => 16,
            0x04 => 32,
            0x05 => 64,
            0x06 => 128,
            0x07 => 256,
            0x08 => 512,
            _ => panic!("Invalid ROM size in header")
        }
    }

    pub fn num_ram_banks(&self) -> usize {
        match self.ram_size {
            0x00 => 0,
            0x01 => 0, // Unused
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ =>panic!("Invalid RAM size in header")
        }
    }

    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn cartridge_type(&self) -> u8 {
        self.cartridge_type
    }

}