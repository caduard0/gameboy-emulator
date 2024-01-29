// https://gbdev.io/pandocs/Memory_Map.html
use core::ops::{ BitOr, BitAnd };
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

impl BitAnd for Flag {
    type Output = u8;
    fn bitand(self, other: Flag) -> Self::Output {
        (self as u8) & (other as u8)
    }
}
impl BitOr for Flag {
    type Output = u8;
    fn bitor(self, other: Flag) -> Self::Output {
        (self as u8) | (other as u8)
    }
}

pub struct Processor {
    pub stack_pointer: u16,
    program_counter: u16,

    registers: Box<[u8; 8]>,

    memory: Box<[u8; 0x1_0000]>,
}

/* 
 * [0000, 3FFF] ROM Bank 00 -> Cartridge
 * [4000, 7FFF] ROM Bank 01~NN -> Cartridge
 * [8000, 9FFF] VRAM
 * [A000, BFFF] External RAM -> Cartridge
 * [C000, CFFF] Work RAM
 * [D000, DFFF] Work RAM
 * [E000, FDFF] ECHO RAM - Mirror of C000~DDFF - Prohibited Access
 * [FE00, FE9F] OAM
 * [FEA0, FEFF] Not Usable - Prohibited
 * [FF00, FF7F] I/O Registers
 * [FF80, FFFE] High RAM
 * [FFFF] Interrupt Enable Register
 */

// Cartridge Header => [0100, 014F]
/* [0100, 0103] Entry Point
 * [0104, 0133] Nintendo Logo - MUST HAVE
 * [0134, 0143] Game Tittle - OLD Cartridges
 * [013F, 0142] Manifacturer Code - NEWER Cartridges
 * [0143] CGB Flag - NEWER Cartridges
 * [0144, 0145] New Licensee Code
 * [0146] SGB Flag
 * [0147] Cartridge Type
 * [0148] ROM size
 * [0149] RAM size
 * [014A] Destination code
 * [014B] Old Licensee Code
 * [014C] ROM version number
 * [014D] Header Checksum
 * [014E, 014F] Global Checksum
 */

impl Processor {
    pub fn new() -> Self {
        Processor {
            stack_pointer: 0,
            program_counter: 0,
            registers: Box::new([0; 8]),
            memory: Box::new([0; 0x1_0000]),
        }
    }

    pub fn load_cartridge(&mut self, path: &str) {
        let global_path = Path::new(path);

        let mut file = match File::open(&global_path) {
            Err(why) => panic!("couldn't open {}: {}", global_path.display(),why),
            Ok(file) => file,
        };

        let mut bytes: Vec<u8> = vec![0; 0x80_0000];
        match file.read(&mut bytes) {
            Err(why) => panic!("couldn't read: {}", why),
            Ok(_) => (),
        }

        let tittle = &bytes[0x0134..0x0143];
        println!("Game Tittle: {}", std::str::from_utf8(tittle).unwrap());

        // Check if header is correct
        let mut sum: u8 = 0;
        for i in 0x0134..=0x014C {
            sum = sum.wrapping_sub(bytes[i].wrapping_add(1));
        }
        assert_eq!(sum,bytes[0x14D], "Cartridge corrupted");

        // Get cartridge type
        match bytes[0x147] {
            0x00 => println!("only ROM"),
            0x01 => println!("MBC1"),
            0x02 => println!("MBC1+RAM"),
            0x03 => println!("MBC1+RAM+BATTERY"),
            0x05 => println!("MBC2"),
            0x06 => println!("MBC2+BATTERY"),
            0x08 => println!("ROM+RAM"),
            0x09 => println!("ROM+RAM+BATTERY"),
            0x0B => println!("MMM01"),
            0x0C => println!("MMM01+RAM"),
            0x0D => println!("MMM01+RAM+BATTERY"),
            0x0F => println!("MBC3+TIMER+BATTERY"),
            0x10 => println!("MBC3+TIMER+RAM+BATTERY"),
            0x11 => println!("MBC3"),
            0x12 => println!("MBC3+RAM"),
            0x13 => println!("MBC3+RAM+BATTERY"),
            0x19 => println!("MBC5"),
            0x1A => println!("MBC5+RAM"),
            0x1B => println!("MBC5+RAM+BATTERY"),
            0x1C => println!("MBC5+RUMBLE"),
            0x1D => println!("MBC5+RUMBLE+RAM"),
            0x1E => println!("MBC5+RUMBLE+RAM+BATTERY"),
            0x20 => println!("MBC6"),
            0x22 => println!("MBC7+SENSOR+RUMBLE+RAM+BATTERY"),
            0xFC => println!("POCKET CAMERA"),
            0xFD => println!("BANDAI TAMA5"),
            0xFE => println!("HuC3"),
            0xFF => println!("HuC1+RAM+BATTERY"),
            _ => panic!("invalid cartridge type"),
        }

        // Get cartridge RAM size
        match bytes[0x149] {
            0x00 => println!("no RAM"),
            0x01 => println!("Unused"),
            0x02 => println!("8 KiB - 1 bank"),
            0x03 => println!("32 KiB - 4 banks"),
            0x04 => println!("128 KiB - 16 banks"),
            0x05 => println!("64 KiB - 8 banks"),
            _ => panic!("invalid cartridge RAM type"),
        }

        // TEMP for ONLY ROM
        assert_eq!(bytes[0x147], 0);
        self.memory[0..8000].copy_from_slice(&bytes[0..8000]);
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

    pub fn read_flags(&self) -> u8 {
        self.read_register(Register::F)
    }

    pub fn write_memory(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
    
    pub fn read_memory(&mut self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}


