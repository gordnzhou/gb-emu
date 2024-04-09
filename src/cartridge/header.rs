use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, Read, Seek, SeekFrom};

const HEADER_SIZE: usize = 0x50;
const HEADER_START: usize = 0x100;

const LOGO_BYTES: usize = 0x30;

const CGB_ENHANCED: u8 = 0x80;
const CGB_ONLY: u8 = 0xC0;

#[derive(Hash)]
pub struct Header {
    nintendo_logo: [u8; LOGO_BYTES],
    manufacturer_code: String,
    cgb_flag: u8,
    licensee_code: u16,
    sgb_flag: u8,
    destination_code: u8, 
    version_number: u8,
    header_checksum: u8, 
    global_checksum: u16,

    title: String,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8, 
}

impl Header {
    pub fn from_file(rom_path: &str) -> io::Result<Self> {
        let mut file = File::open(rom_path)?;
        file.seek(SeekFrom::Start(HEADER_START as u64))?;
        let mut header_bytes = [0; HEADER_SIZE];
        file.read_exact(&mut header_bytes)?;

        Ok(Header::new(header_bytes))
    }

    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut header_bytes = [0; HEADER_SIZE];
        for i in 0..HEADER_SIZE {
            header_bytes[i] = bytes[HEADER_START + i];
        }

        Header::new(header_bytes)
    }

    /// Constructs a header using header_bytes (from addresses 0x0100 to 0x014F)
    pub fn new(header_bytes: [u8; HEADER_SIZE]) -> Self {
        let nintendo_logo = header_bytes[0x04..=0x33].try_into().unwrap();

        let cgb_flag = header_bytes[0x43];
        let mut title_end = 0x43;
        if cgb_flag == CGB_ENHANCED || cgb_flag == CGB_ONLY {
            // byte at 0x143 is used for CGB flag instead of title in this case
            title_end -= 1; 
        }
        let title_bytes = header_bytes[0x34..=title_end].to_vec();
        let title = match String::from_utf8(title_bytes) {
            Ok(s) => s.replace("\0", ""),
            Err(e) => panic!("Unable to parse header title: {}", e),
        };

        let manufacturer_code = match String::from_utf8(header_bytes[0x3F..=0x42].to_vec()) {
            Ok(s) => s.replace("\0", ""),
            Err(_) => String::from(""),
        };

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

        let destination_code = header_bytes[0x4A];

        let version_number = header_bytes[0x4C];

        let header_checksum = header_bytes[0x4D];

        // unused for most games
        let global_checksum = ((header_bytes[0x4E] as u16) << 8) | header_bytes[0x4F] as u16;

        let mut checksum: u8 = 0;
        for i in 0x34..=0x4C {
            checksum = checksum.wrapping_sub(header_bytes[i]).wrapping_sub(1);
        }
        assert!(checksum == header_checksum, "Header bytes do not match header checksum.");

        Header {
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
        }
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

    pub fn cgb_compatible(&self) -> bool {
        self.cgb_flag & 0x80 !=  0
    }

    pub fn get_hash_string(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish().to_string()
    }
}