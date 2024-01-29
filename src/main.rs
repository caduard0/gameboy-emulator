#![allow(dead_code)]

pub mod processor;
pub mod instructions;

use crate::processor::{Processor, Register};
use crate::instructions::*;

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


pub struct Cartridge {
    rom: Box<[[u8; 0x4000]]>,
    ram: Box<[[u8; 0x2000]]>,
    rom_banks: usize,
    ram_banks: usize,
}

impl Cartridge {
    fn new(rom_banks: usize, ram_banks: usize) -> Cartridge {
        assert!(rom_banks >= 2, "rom size too low");
        assert!(rom_banks <= 512, "out of bounds rom size");
        assert!(ram_banks <= 16, "out of bounds ram size");

        Cartridge {
            rom: vec![[0; 0x4000]; rom_banks].into_boxed_slice(),
            ram: vec![[0; 0x2000]; ram_banks].into_boxed_slice(),
            rom_banks,
            ram_banks,
        }
    }
}

fn main() {
    let mut cpu = Processor::new();

    cpu.load_cartridge("games/Tetris.gb");

}
