#[derive(Clone, Copy)]
pub struct Regs {
    pub a: u8,
    f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

#[derive(Copy, Clone)]
pub enum ProcessorFlag
{
    C = 0b00010000,
    H = 0b00100000,
    N = 0b01000000,
    Z = 0b10000000,
}

impl Regs {
    pub fn new() -> Regs {
        use ProcessorFlag::*;
        Regs {
            a: 0x01,
            f: C as u8 | H as u8 | Z as u8,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    // Getters.

    pub fn get_af(&self) -> u16 {
        return ((self.a as u16) << 8) | ((self.f & 0xF0) as u16);
    }

    pub fn get_bc(&self) -> u16 {
        return ((self.b as u16) << 8) | (self.c as u16);
    }

    pub fn get_de(&self) -> u16 {
        return ((self.d as u16) << 8) | (self.e as u16);
    }

    pub fn get_hl(&self) -> u16 {
        return ((self.h as u16) << 8) | (self.l as u16);
    }

    // Setters.

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value & 0x00F0) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8;
    }
}

pub struct Microprocessor {
    regs: Regs,
}

impl Microprocessor {
    pub fn new() -> Microprocessor {
        Microprocessor {
            regs: Regs::new(),
        }
    }

    pub fn fetch_next_instruction(&mut self) -> u16 {
        
    }
}