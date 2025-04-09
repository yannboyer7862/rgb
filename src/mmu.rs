const VRAM_SIZE: usize = 0x2000; // 8192
const WRAM_SIZE: usize = 0x2000;
const ERAM_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0x9F;
const HRAM_SIZE: usize = 0x7F;


pub struct MMU {
    vram: [u8; VRAM_SIZE],
    wram: [u8; WRAM_SIZE],
    eram: [u8; ERAM_SIZE],
    oam: [u8; OAM_SIZE],
    hram: [u8; HRAM_SIZE],
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            vram: [0x0; VRAM_SIZE],
            wram: [0x0; WRAM_SIZE],
            eram: [0x0; ERAM_SIZE],
            oam: [0x0; OAM_SIZE],
            hram: [0x0; HRAM_SIZE],
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {

    }

    pub fn write_byte(&self, addr: u16, value: u8) {

    }

    pub fn read_word(&self, addr: u16) -> u16 {

    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        
    }
}