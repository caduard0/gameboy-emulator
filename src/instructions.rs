use crate::{Processor, processor::Register, processor::Flag};

pub fn to_u16(b0: u8, b1: u8) -> u16 {
    ((b0 as u16) << 8) | b1 as u16
}

pub fn to_u8(b: u16) -> (u8, u8) {
    ((b >> 8) as u8, b as u8)
}

pub struct Instruction {
    pub operation: fn(&mut Processor, u16),
    pub bytes: u8,
    pub cycles: u8,
}

// n  -> unsigned 8 bit data
// nn -> unsigned 16 bit data
// e  -> signed 8 bit
//
// nop -> no operation
// ld -> load operation
// inc -> increase by 1 operation
// dec -> decrease by 1 operation

/*
const INSTRUCTIONS: [Instruction; 256] = 
[
    Instruction { operation: nop, 1, 4 },
    Instruction { operation: ld_bc_nn, 3, 12 },
    Instruction { operation: ld_bc_a, 1, 8 },
    Instruction { operation: inc_bc, 1, 8 },
    Instruction { operation: inc_b, 1, 8 },
    Instruction { operation: dec_b,
    Instruction { operation: ld_b_n,
    Instruction { operation: rlca,
    Instruction { operation: ld_nn_sp,
    Instruction { operation: add_hl_bc,
    Instruction { operation: ld_a_bc,
    Instruction { operation: dec_bc,
    Instruction { operation: inc_c,
    Instruction { operation: dec_c,
    Instruction { operation: ld_c_n,
    Instruction { operation: rrca, 
    Instruction { operation: stop,
    Instruction { operation: ld_de_nn,
    Instruction { operation: ld_de_a,
    Instruction { operation: inc_de, 
    Instruction { operation: inc_d,
    Instruction { operation: dec_d,
    Instruction { operation: ld_d_n, 
    Instruction { operation: rla,
    Instruction { operation: jr_e,
    Instruction { operation: add_hl_de,
    Instruction { operation: ld_a_de,
    Instruction { operation: dec_de,
    Instruction { operation: inc_e,
    Instruction { operation: dec_e,
    Instruction { operation: ld_e_n, 
    Instruction { operation: rra, 
    Instruction { operation: jr_nz_e,
    Instruction { operation: ld_hl_nn,
    Instruction { operation: ld_hlp_a,
    Instruction { operation: inc_hl, 
    Instruction { operation: inc_h,
    Instruction { operation: dec_h,
    Instruction { operation: ld_h_n, 
    Instruction { operation: daa,
    Instruction { operation: jr_z_e,
    Instruction { operation: add_hl_hl,
    Instruction { operation: ld_a_hlp,
    Instruction { operation: dec_hl,
    Instruction { operation: inc_l,
    Instruction { operation: dec_l,
    Instruction { operation: ld_l_n, 
    Instruction { operation: cpl, 
    Instruction { operation: jr_nc_e,
    Instruction { operation: ld_sp_nn,
    Instruction { operation: ld_hlm_a,
    Instruction { operation: inc_sp,
    Instruction { operation: inc_hl,
    Instruction { operation: dec_hl,
    Instruction { operation: ld_hl_n,
    Instruction { operation: scf,
    Instruction { operation: jr_c_e,
    Instruction { operation: add_hl_sp,
    Instruction { operation: ld_a_hlm,
    Instruction { operation: dec_sp,
    Instruction { operation: inc_a,
    Instruction { operation: dec_a,
    Instruction { operation: ld_a_n, 
    Instruction { operation: cff, 
];
*/


pub fn nop(_cpu: &mut Processor, _instruction: u16) { }

/// Load `nn` to BC
pub fn ld_bc_nn(cpu: &mut Processor, instruction: u16) {
    let (b, c) = to_u8(instruction);
    cpu.write_register(Register::B, b);
    cpu.write_register(Register::C, c);
}

/// Load A (accumulator) to the address stored at BC
pub fn ld_bc_a(cpu: &mut Processor, _instruction: u16) {
    let address = to_u16(cpu.read_register(Register::B), cpu.read_register(Register::C));
    let a = cpu.read_register(Register::A);
    cpu.write_memory(address, a);
}

/// Increase BC by 1 
pub fn inc_bc(cpu: &mut Processor, _instruction: u16) {
    let (b, c) = to_u8(to_u16(cpu.read_register(Register::B), cpu.read_register(Register::C)) + 1);
    cpu.write_register(Register::B, b);
    cpu.write_register(Register::C, c);
}

/// Increase B by 1 
pub fn inc_b(cpu: &mut Processor, _instruction: u16) {
    let b = cpu.read_register(Register::B).checked_add(1).expect("overflow");
    cpu.write_register(Register::B, b);

    // Calculate half carry bit
    let h = ((b-1) & 0xF) + (b & 0xF) & 0x10;

    // Set zero flag if zero
    if b == 0 { cpu.set_flag(Flag::Z); }
    // Reset subtraction flag
    cpu.reset_flag(Flag::N);
    // Set Half Carry
    if h == 0x10 { cpu.set_flag(Flag::H); }
}

/// Decrease B by 1 
pub fn dec_b(cpu: &mut Processor, _instruction: u16) {
    let b = cpu.read_register(Register::B).checked_sub(1).expect("underflow");
    cpu.write_register(Register::B, b);

    // Calculate half carry bit
    let h = ((b-1) & 0xF) + (b & 0xF) & 0x10;

    // Set zero flag if zero
    if b == 0 { cpu.set_flag(Flag::Z); }
    // Set subtraction flag
    cpu.set_flag(Flag::N);
    // Set Half Carry
    if h == 0x10 { cpu.set_flag(Flag::H); }
}

/// Load `n` to B
pub fn ld_b_n(cpu: &mut Processor, instruction: u16) {
    let b = (instruction >> 8) as u8;
    cpu.write_register(Register::B, b);
}

pub fn rlca(cpu: &mut Processor, instruction: u16) {
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ld_bc_nn_works() {
        let mut cpu = Processor::new();
        ld_bc_nn(&mut cpu, 0xABCD);

        let value = to_u16(cpu.read_register(Register::B), cpu.read_register(Register::C));
        assert_eq!(value, 0xABCD);
    }

    #[test]
    fn ld_bc_a_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::A, 0xFE);
        cpu.write_register(Register::B, 0xAB);
        cpu.write_register(Register::C, 0xCD);
        ld_bc_a(&mut cpu, 0);

        // OUT OF BOUNDS
        let value = cpu.read_memory(0xABCD);
        assert_eq!(value, 0xFE);
    }
}