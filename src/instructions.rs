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

// !!! LITTLE ENDIAN !!!
// r8  -> 8 bit register
// r16 -> 16 bit register
// n8  -> 8 bit constant
// n16 -> 16 bit constant
// e8  -> 8 bit signed offset
// u3  -> 3 bit unsigned constant
// cc  -> condition code
// vec -> one of the RST vectors (0x00, 0x08, 0x10, 0x18, 0x20, 0x28, 0x30, and 0x38)
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
    Instruction { operation: dec_b, 1, 4 },
    Instruction { operation: ld_b_n, 2, 8 },
    Instruction { operation: rlca, 1, 4 },
    Instruction { operation: ld_n16_sp, 3, 20 },
    Instruction { operation: add_hl_bc, 1, 8 },
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
    Instruction { operation: ld_hli_a,
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
    Instruction { operation: ld_hld_a,
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

/// Load `n16` to BC
pub fn ld_bc_n16(cpu: &mut Processor, instruction: u16) {
    let (c, b) = to_u8(instruction);
    cpu.write_register(Register::B, b);
    cpu.write_register(Register::C, c);
}

/// Load A (accumulator) to the address pointed by BC
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

/// Increase C by 1 
pub fn inc_c(cpu: &mut Processor, _instruction: u16) {
    let c = cpu.read_register(Register::C).checked_add(1).expect("overflow");
    cpu.write_register(Register::C, c);

    // Calculate half carry bit
    let h = ((c-1) & 0xF) + (c & 0xF) & 0x10;

    // Set zero flag if zero
    if c == 0 { cpu.set_flag(Flag::Z); }
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
    let h = b & 0x0F;

    // Set zero flag if zero
    if b == 0 { cpu.set_flag(Flag::Z); }
    // Set subtraction flag
    cpu.set_flag(Flag::N);
    // Set Half Carry
    if h == 0x00 { cpu.set_flag(Flag::H); }
}

/// Decrease C by 1 
pub fn dec_c(cpu: &mut Processor, _instruction: u16) {
    let c = cpu.read_register(Register::C).checked_sub(1).expect("underflow");
    cpu.write_register(Register::C, c);

    // Calculate half carry bit
    let h = c & 0x0F;

    // Set zero flag if zero
    if c == 0 { cpu.set_flag(Flag::Z); }
    // Set subtraction flag
    cpu.set_flag(Flag::N);
    // Set Half Carry
    if h == 0x00 { cpu.set_flag(Flag::H); }
}

/// Load `n8` to B
pub fn ld_b_n8(cpu: &mut Processor, instruction: u16) {
    let b = (instruction >> 8) as u8;
    cpu.write_register(Register::B, b);
}

/// Load `n8` to C
pub fn ld_c_n8(cpu: &mut Processor, instruction: u16) {
    let c = (instruction >> 8) as u8;
    cpu.write_register(Register::C, c);
}

/// Rotate A (accumulator) left
pub fn rlca(cpu: &mut Processor, _instruction: u16) {
    let a = cpu.read_register(Register::A);
    let carry = a >> 7;
    if carry == 0x1 {
        cpu.set_flag(Flag::C);
    } else {
        cpu.reset_flag(Flag::C);
    }
    cpu.write_register(Register::A, (a << 1) | carry);
}

/// Rotate A (accumulator) right
pub fn rrca(cpu: &mut Processor, _instruction: u16) {
    let a = cpu.read_register(Register::A);
    let carry = a << 7;
    if carry == 0x80 {
        cpu.set_flag(Flag::C);
    } else {
        cpu.reset_flag(Flag::C);
    }
    cpu.write_register(Register::A, (a >> 1) | carry);
}

/// Load SP into address pointed by `n16`
pub fn ld_n16_sp(cpu: &mut Processor, instruction: u16) {
    let b0 = (cpu.stack_pointer as u8) & 0xFF;
    let b1 = (cpu.stack_pointer >> 8) as u8;

    let big_endian = to_u16((instruction & 0xFF) as u8, (instruction>>8) as u8);

    cpu.write_memory(big_endian+1, b0);
    cpu.write_memory(big_endian, b1);
}

/// Add BC to HL
pub fn add_hl_bc(cpu: &mut Processor, _instruction: u16) {
    let bc = to_u16(cpu.read_register(Register::B), cpu.read_register(Register::C));
    let hl = to_u16(cpu.read_register(Register::H), cpu.read_register(Register::L));
    
    let (b0, b1) = to_u8(hl + bc);

    cpu.write_register(Register::H, b0);
    cpu.write_register(Register::L, b1);
}

/// Load value pointed by BC to A (accumulator)
pub fn ld_a_bc(cpu: &mut Processor, _instruction: u16) {
    let bc = cpu.read_memory(to_u16(cpu.read_register(Register::B), cpu.read_register(Register::C)));
    cpu.write_register(Register::A, bc);
}

/// Decrease BC by 1 
pub fn dec_bc(cpu: &mut Processor, _instruction: u16) {
    let (b, c) = to_u8(to_u16(cpu.read_register(Register::B), cpu.read_register(Register::C)) - 1);
    cpu.write_register(Register::B, b);
    cpu.write_register(Register::C, c);
}





#[cfg(test)]
mod test {
    use crate::ld_a_bc;

    use super::*;

    #[test]
    fn nop_works() {
        let mut cpu = Processor::new();
        nop(&mut cpu, 0);

        assert_eq!(0, 0);
    }

    #[test]
    fn ld_bc_n16_works() {
        let mut cpu = Processor::new();
        // little endian
        ld_bc_n16(&mut cpu, 0x00_01);

        let value = to_u16(cpu.read_register(Register::B), cpu.read_register(Register::C));
        assert_eq!(value, 0x0100);
    }

    #[test]
    fn ld_bc_a_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::A, 0xFE);
        cpu.write_register(Register::B, 0x01);
        cpu.write_register(Register::C, 0x00);
        ld_bc_a(&mut cpu, 0);

        let value = cpu.read_memory(0x0100);
        assert_eq!(value, 0xFE);
    }

    #[test]
    fn inc_bc_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::B, 0x01);
        cpu.write_register(Register::C, 0x00);
        inc_bc(&mut cpu, 0);

        let value = to_u16(cpu.read_register(Register::B), cpu.read_register(Register::C));
        assert_eq!(value, 0x0101);
    }

    #[test]
    fn dec_bc_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::B, 0x01);
        cpu.write_register(Register::C, 0x00);
        dec_bc(&mut cpu, 0);

        let value = to_u16(cpu.read_register(Register::B), cpu.read_register(Register::C));
        assert_eq!(value, 0x00FF);
    }

    #[test]
    fn inc_b_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::B, 0x01);
        inc_b(&mut cpu, 0);

        let value = cpu.read_register(Register::B);
        assert_eq!(value, 0x02);
    }

    #[test]
    fn inc_c_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::C, 0x01);
        inc_c(&mut cpu, 0);

        let value = cpu.read_register(Register::C);
        assert_eq!(value, 0x02);
    }

    #[test]
    fn dec_b_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::B, 0x02);
        dec_b(&mut cpu, 0);

        let value = cpu.read_register(Register::B);
        assert_eq!(value, 0x01);
    }

    #[test]
    fn dec_c_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::C, 0x02);
        dec_c(&mut cpu, 0);

        let value = cpu.read_register(Register::C);
        assert_eq!(value, 0x01);
    }

    #[test]
    fn ld_b_n8_works() {
        let mut cpu = Processor::new();
        ld_b_n8(&mut cpu, 0xFE00);

        let value = cpu.read_register(Register::B);
        assert_eq!(value, 0xFE);
    }

    #[test]
    fn ld_c_n8_works() {
        let mut cpu = Processor::new();
        ld_c_n8(&mut cpu, 0xFE00);

        let value = cpu.read_register(Register::C);
        assert_eq!(value, 0xFE);
    }

    #[test]
    fn rlca_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::A, 0b11001100);
        rlca(&mut cpu, 0);

        let value = cpu.read_register(Register::A);
        assert_eq!(value, 0b10011001);
    }

    #[test]
    fn rrca_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::A, 0b11001100);
        rrca(&mut cpu, 0);

        let value = cpu.read_register(Register::A);
        assert_eq!(value, 0b01100110);
    }


    #[test]
    fn ld_n16_sp_works() {
        let mut cpu = Processor::new();
        cpu.stack_pointer = 0xFE69;
        // Little endian
        ld_n16_sp(&mut cpu, 0x00_01);

        let value = to_u16(cpu.read_memory(0x0100), cpu.read_memory(0x0101));
        assert_eq!(value, 0xFE69);
    }

    #[test]
    fn add_hl_bc_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::H, 0xAB);
        cpu.write_register(Register::L, 0xCD);
        cpu.write_register(Register::B, 0x11);
        cpu.write_register(Register::C, 0x11);
        add_hl_bc(&mut cpu, 0);

        let value = to_u16(cpu.read_register(Register::H), cpu.read_register(Register::L));
        assert_eq!(value, 0xBCDE);
    }


    #[test]
    fn ld_a_bc_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::B, 0x01);
        cpu.write_register(Register::C, 0x00);
        cpu.write_memory(0x0100, 0xFE);
        ld_a_bc(&mut cpu, 0);

        let value = cpu.read_register(Register::A);
        assert_eq!(value, 0xFE);
    }
}
