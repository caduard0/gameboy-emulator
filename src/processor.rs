use core::ops::BitOr;

pub enum Register {
    A = 0,
    F = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    H = 6,
    L = 7,
}

pub enum Flag { 
    Z  = 0b1000_0000,
    N  = 0b0100_0000,
    H  = 0b0010_0000,
    C  = 0b0001_0000,
}

impl BitOr for Flag {
    type Output = u8;
    fn bitor(self, other: Flag) -> Self::Output {
        (self as u8) | (other as u8)
    }
}

pub struct Processor {
    stack_pointer: u16,
    program_counter: u16,

    registers: Box<[u8; 8]>,

    ram: Box<[u8; 0x2000]>,
    vram: Box<[u8; 0x2000]>,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            stack_pointer: 0,
            program_counter: 0,
            registers: Box::new([0; 8]),
            ram: Box::new([0; 0x2000]),
            vram: Box::new([0; 0x2000]),
        }
    }


    pub fn write_register(&mut self, index: Register, value: u8) {
        self.registers[index as usize] = value;
    }
    
    pub fn read_register(&self, index: Register) -> u8 {
        self.registers[index as usize]
    }

    pub fn set_flag(&mut self, flag: Flag) {
        self.write_register(Register::F, flag as u8);
    }

    pub fn reset_flag(&mut self, flag: Flag) {
        self.write_register(Register::F, !(flag as u8));
    }

    pub fn read_flag(&self) -> u8 {
        self.read_register(Register::F)
    }

    pub fn write_memory(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }
    
    pub fn read_memory(&mut self, address: u16) -> u8 {
        self.ram[address as usize]
    }
}


