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


/// NOP
pub fn nop(_cpu: &mut Processor, _instruction: u16) { }

/// INC r8
pub fn increase_register(cpu: &mut Processor, _instruction: u16, register: Register) {
    // Increment register by 1 (overflow panics)
    let register_increment = cpu.read_register(register).checked_add(1).expect("overflow");
    cpu.write_register(register, register_increment );

    // Calculate half carry bit
    let h = ((register_increment -1) & 0xF) + (register_increment & 0xF) & 0x10;

    // Set zero flag if zero
    if register_increment == 0 { cpu.set_flag(Flag::Z); }
    // Reset subtraction flag
    cpu.reset_flag(Flag::N);
    // Set Half Carry
    if h == 0x10 { cpu.set_flag(Flag::H); }
}

pub fn inc_a(cpu: &mut Processor, _instruction: u16) { increase_register(cpu, _instruction, Register::A); }
pub fn inc_b(cpu: &mut Processor, _instruction: u16) { increase_register(cpu, _instruction, Register::B); }
pub fn inc_c(cpu: &mut Processor, _instruction: u16) { increase_register(cpu, _instruction, Register::C); }
pub fn inc_d(cpu: &mut Processor, _instruction: u16) { increase_register(cpu, _instruction, Register::D); }
pub fn inc_e(cpu: &mut Processor, _instruction: u16) { increase_register(cpu, _instruction, Register::E); }
pub fn inc_h(cpu: &mut Processor, _instruction: u16) { increase_register(cpu, _instruction, Register::H); }
pub fn inc_l(cpu: &mut Processor, _instruction: u16) { increase_register(cpu, _instruction, Register::L); }

/// INC [HL]
pub fn inc_hlp(cpu: &mut Processor, _instruction: u16) {
    // Get position pointed by HL
    let memory_position = to_u16(cpu.read_register(Register::H), cpu.read_register(Register::L));

    // Increment register by 1 (overflow panics)
    let register_increment = cpu.read_memory(memory_position).checked_add(1).expect("overflow");
    cpu.write_memory(memory_position, register_increment);

    // Calculate half carry bit
    let h = ((register_increment -1) & 0xF) + (register_increment & 0xF) & 0x10;

    // Set zero flag if zero
    if register_increment == 0 { cpu.set_flag(Flag::Z); }
    // Reset subtraction flag
    cpu.reset_flag(Flag::N);
    // Set Half Carry
    if h == 0x10 { cpu.set_flag(Flag::H); }
}

/// DEC r8
pub fn decrease_register(cpu: &mut Processor, _instruction: u16, register: Register) {
    // Increment register by 1 (overflow panics)
    let register_decrement = cpu.read_register(register).checked_sub(1).expect("underflow");
    cpu.write_register(register, register_decrement );

    // Calculate half carry bit
    let h = register_decrement & 0x0F;

    // Set zero flag if zero
    if register_decrement == 0 { cpu.set_flag(Flag::Z); }
    // Set subtraction flag
    cpu.set_flag(Flag::N);
    // Set Half Carry
    if h == 0x00 { cpu.set_flag(Flag::H); }
}

pub fn dec_a(cpu: &mut Processor, _instruction: u16) { decrease_register(cpu, _instruction, Register::A); }
pub fn dec_b(cpu: &mut Processor, _instruction: u16) { decrease_register(cpu, _instruction, Register::B); }
pub fn dec_c(cpu: &mut Processor, _instruction: u16) { decrease_register(cpu, _instruction, Register::C); }
pub fn dec_d(cpu: &mut Processor, _instruction: u16) { decrease_register(cpu, _instruction, Register::D); }
pub fn dec_e(cpu: &mut Processor, _instruction: u16) { decrease_register(cpu, _instruction, Register::E); }
pub fn dec_h(cpu: &mut Processor, _instruction: u16) { decrease_register(cpu, _instruction, Register::H); }
pub fn dec_l(cpu: &mut Processor, _instruction: u16) { decrease_register(cpu, _instruction, Register::L); }

/// DEC [HL]
pub fn dec_hlp(cpu: &mut Processor, _instruction: u16) {
    // Get position pointed by HL
    let memory_position = to_u16(cpu.read_register(Register::L), cpu.read_register(Register::H));

    // Decrement register by 1 (overflow panics)
    let register_decrement = cpu.read_memory(memory_position).checked_sub(1).expect("underflow");
    cpu.write_memory(memory_position, register_decrement );

    // Calculate half carry bit
    let h = register_decrement & 0x0F;

    // Set zero flag if zero
    if register_decrement == 0 { cpu.set_flag(Flag::Z); }
    // Set subtraction flag
    cpu.set_flag(Flag::N);
    // Set Half Carry
    if h == 0x00 { cpu.set_flag(Flag::H); }
}

/// INC r16
pub fn increase_double_register(cpu: &mut Processor, _instruction: u16, register_a: Register, register_b: Register) {
    // Get position pointed by HL
    let register_value = to_u16(cpu.read_register(register_a), cpu.read_register(register_b));

    // Increment register by 1 (overflow panics)
    let register_increment = register_value.checked_add(1).expect("overflow");
    let (value_a, value_b) = to_u8(register_increment);
    cpu.write_register(register_a, value_a);
    cpu.write_register(register_b, value_b);
}

pub fn inc_bc(cpu: &mut Processor, _instruction: u16) { increase_double_register(cpu, _instruction, Register::B, Register::C); }
pub fn inc_de(cpu: &mut Processor, _instruction: u16) { increase_double_register(cpu, _instruction, Register::D, Register::E); }
pub fn inc_hl(cpu: &mut Processor, _instruction: u16) { increase_double_register(cpu, _instruction, Register::H, Register::L); }

/// INC SP
pub fn inc_sp(cpu: &mut Processor, _instruction: u16) {
    // Increment register by 1 (overflow panics)
    let register_increment = cpu.stack_pointer.checked_add(1).expect("overflow");
    cpu.stack_pointer = register_increment;
}

/// DEC r16
pub fn decrease_double_register(cpu: &mut Processor, _instruction: u16, register_a: Register, register_b: Register) {
    // Get position pointed by HL
    let register_value = to_u16(cpu.read_register(register_a), cpu.read_register(register_b));

    // Increment register by 1 (overflow panics)
    let register_increment = register_value.checked_sub(1).expect("overflow");
    let (value_a, value_b) = to_u8(register_increment);
    cpu.write_register(register_a, value_a);
    cpu.write_register(register_b, value_b);
}

pub fn dec_bc(cpu: &mut Processor, _instruction: u16) { decrease_double_register(cpu, _instruction, Register::B, Register::C); }
pub fn dec_de(cpu: &mut Processor, _instruction: u16) { decrease_double_register(cpu, _instruction, Register::D, Register::E); }
pub fn dec_hl(cpu: &mut Processor, _instruction: u16) { decrease_double_register(cpu, _instruction, Register::H, Register::L); }

/// DEC SP
pub fn dec_sp(cpu: &mut Processor, _instruction: u16) {
    // Increment register by 1 (overflow panics)
    let register_increment = cpu.stack_pointer.checked_sub(1).expect("overflow");
    cpu.stack_pointer = register_increment;
}

/// LD r8 r8
pub fn load_register_register(cpu: &mut Processor, _instruction: u16, register_in: Register, register_out: Register) {
    cpu.write_register(register_in, cpu.read_register(register_out));
}

pub fn ld_b_b(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::B, Register::B); }
pub fn ld_b_c(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::B, Register::C); }
pub fn ld_b_d(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::B, Register::D); }
pub fn ld_b_e(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::B, Register::E); }
pub fn ld_b_h(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::B, Register::H); }
pub fn ld_b_l(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::B, Register::L); }
pub fn ld_b_a(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::B, Register::A); }

pub fn ld_c_b(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::C, Register::B); }
pub fn ld_c_c(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::C, Register::C); }
pub fn ld_c_d(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::C, Register::D); }
pub fn ld_c_e(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::C, Register::E); }
pub fn ld_c_h(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::C, Register::H); }
pub fn ld_c_l(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::C, Register::L); }
pub fn ld_c_a(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::C, Register::A); }

pub fn ld_d_b(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::D, Register::B); }
pub fn ld_d_c(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::D, Register::C); }
pub fn ld_d_d(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::D, Register::D); }
pub fn ld_d_e(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::D, Register::E); }
pub fn ld_d_h(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::D, Register::H); }
pub fn ld_d_l(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::D, Register::L); }
pub fn ld_d_a(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::D, Register::A); }

pub fn ld_e_b(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::E, Register::B); }
pub fn ld_e_c(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::E, Register::C); }
pub fn ld_e_d(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::E, Register::D); }
pub fn ld_e_e(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::E, Register::E); }
pub fn ld_e_h(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::E, Register::H); }
pub fn ld_e_l(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::E, Register::L); }
pub fn ld_e_a(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::E, Register::A); }

pub fn ld_h_b(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::H, Register::B); }
pub fn ld_h_c(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::H, Register::C); }
pub fn ld_h_d(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::H, Register::D); }
pub fn ld_h_e(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::H, Register::E); }
pub fn ld_h_h(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::H, Register::H); }
pub fn ld_h_l(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::H, Register::L); }
pub fn ld_h_a(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::H, Register::A); }

pub fn ld_l_b(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::L, Register::B); }
pub fn ld_l_c(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::L, Register::C); }
pub fn ld_l_d(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::L, Register::D); }
pub fn ld_l_e(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::L, Register::E); }
pub fn ld_l_h(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::L, Register::H); }
pub fn ld_l_l(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::L, Register::L); }
pub fn ld_l_a(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::L, Register::A); }

pub fn ld_a_b(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::A, Register::B); }
pub fn ld_a_c(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::A, Register::C); }
pub fn ld_a_d(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::A, Register::D); }
pub fn ld_a_e(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::A, Register::E); }
pub fn ld_a_h(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::A, Register::H); }
pub fn ld_a_l(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::A, Register::L); }
pub fn ld_a_a(cpu: &mut Processor, _instruction: u16) { load_register_register(cpu, _instruction, Register::A, Register::A); }

/// LD r8 n8
pub fn load_register_value(cpu: &mut Processor, instruction: u16, register: Register) {
    let value = (instruction >> 8) as u8;
    cpu.write_register(register, value);
}

pub fn ld_b_n8(cpu: &mut Processor, instruction: u16) { load_register_value(cpu, instruction, Register::B); }
pub fn ld_c_n8(cpu: &mut Processor, instruction: u16) { load_register_value(cpu, instruction, Register::C); }
pub fn ld_d_n8(cpu: &mut Processor, instruction: u16) { load_register_value(cpu, instruction, Register::D); }
pub fn ld_e_n8(cpu: &mut Processor, instruction: u16) { load_register_value(cpu, instruction, Register::E); }
pub fn ld_h_n8(cpu: &mut Processor, instruction: u16) { load_register_value(cpu, instruction, Register::H); }
pub fn ld_l_n8(cpu: &mut Processor, instruction: u16) { load_register_value(cpu, instruction, Register::L); }
pub fn ld_a_n8(cpu: &mut Processor, instruction: u16) { load_register_value(cpu, instruction, Register::A); }

/// LD r16 n16
pub fn load_double_register_value(cpu: &mut Processor, instruction: u16, register_a: Register, register_b: Register) {
    let (value_b, value_a) = to_u8(instruction);
    cpu.write_register(register_a, value_a);
    cpu.write_register(register_b, value_b);
}

pub fn ld_bc_n16(cpu: &mut Processor, instruction: u16) { load_double_register_value(cpu, instruction, Register::B, Register::C); }
pub fn ld_de_n16(cpu: &mut Processor, instruction: u16) { load_double_register_value(cpu, instruction, Register::D, Register::E); }
pub fn ld_hl_n16(cpu: &mut Processor, instruction: u16) { load_double_register_value(cpu, instruction, Register::H, Register::L); }

/// LD SP n16
pub fn ld_sp_n16(cpu: &mut Processor, instruction: u16) {
    let (value_b, value_a) = to_u8(instruction);

    cpu.stack_pointer = to_u16(value_a, value_b);
}

/// LD [HL] r8
pub fn load_hlp_register(cpu: &mut Processor, _instruction: u16, register: Register) {
    let memory_address = to_u16(cpu.read_register(Register::H), cpu.read_register(Register::L));

    cpu.write_memory(memory_address, cpu.read_register(register));
}

pub fn ld_hlp_b(cpu: &mut Processor, _instruction: u16) { load_hlp_register(cpu, _instruction, Register::B); }
pub fn ld_hlp_c(cpu: &mut Processor, _instruction: u16) { load_hlp_register(cpu, _instruction, Register::C); }
pub fn ld_hlp_d(cpu: &mut Processor, _instruction: u16) { load_hlp_register(cpu, _instruction, Register::D); }
pub fn ld_hlp_e(cpu: &mut Processor, _instruction: u16) { load_hlp_register(cpu, _instruction, Register::E); }
pub fn ld_hlp_h(cpu: &mut Processor, _instruction: u16) { load_hlp_register(cpu, _instruction, Register::H); }
pub fn ld_hlp_l(cpu: &mut Processor, _instruction: u16) { load_hlp_register(cpu, _instruction, Register::L); }
pub fn ld_hlp_a(cpu: &mut Processor, _instruction: u16) { load_hlp_register(cpu, _instruction, Register::A); }

/// LD [HL] n8
pub fn ld_hlp_n8(cpu: &mut Processor, instruction: u16) {
    let memory_address = to_u16(cpu.read_register(Register::H), cpu.read_register(Register::L));

    let value = (instruction >> 8) as u8;

    cpu.write_memory(memory_address, value);
}

/// LD r8 [HL]
pub fn load_register_hlp(cpu: &mut Processor, _instruction: u16, register: Register) {
    let memory_address = to_u16(cpu.read_register(Register::H), cpu.read_register(Register::L));
    let value = cpu.read_memory(memory_address);

    cpu.write_register(register, value);
}

pub fn ld_b_hlp(cpu: &mut Processor, _instruction: u16) { load_register_hlp(cpu, _instruction, Register::B); }
pub fn ld_c_hlp(cpu: &mut Processor, _instruction: u16) { load_register_hlp(cpu, _instruction, Register::C); }
pub fn ld_d_hlp(cpu: &mut Processor, _instruction: u16) { load_register_hlp(cpu, _instruction, Register::D); }
pub fn ld_e_hlp(cpu: &mut Processor, _instruction: u16) { load_register_hlp(cpu, _instruction, Register::E); }
pub fn ld_h_hlp(cpu: &mut Processor, _instruction: u16) { load_register_hlp(cpu, _instruction, Register::H); }
pub fn ld_l_hlp(cpu: &mut Processor, _instruction: u16) { load_register_hlp(cpu, _instruction, Register::L); }
pub fn ld_a_hlp(cpu: &mut Processor, _instruction: u16) { load_register_hlp(cpu, _instruction, Register::A); }

/// LD [r16] A
pub fn load_double_registerp_a(cpu: &mut Processor, _instruction: u16, register_a: Register, register_b: Register) {
    let memory_address = to_u16(cpu.read_register(register_a), cpu.read_register(register_b));
    let value = cpu.read_register(Register::A);

    cpu.write_memory(memory_address, value);
}

pub fn ld_bcp_a(cpu: &mut Processor, _instruction: u16) { load_double_registerp_a(cpu, _instruction, Register::B, Register::C); }
pub fn ld_dep_a(cpu: &mut Processor, _instruction: u16) { load_double_registerp_a(cpu, _instruction, Register::B, Register::C); }

// LD [HL+] A
pub fn ld_hli_a(cpu: &mut Processor, _instruction: u16) {
    load_double_registerp_a(cpu, _instruction, Register::H, Register::L);
    inc_hl(cpu, _instruction);
}

// LD [HL-] A
pub fn ld_hld_a(cpu: &mut Processor, _instruction: u16) {
    load_double_registerp_a(cpu, _instruction, Register::H, Register::L);
    dec_hl(cpu, _instruction);
}

// LD [r16] A
pub fn ld_r16_a(cpu: &mut Processor, instruction: u16) {
    let (address_b, address_a) = to_u8(instruction);
    let memory_address = to_u16(address_a, address_b);

    cpu.write_memory(memory_address, cpu.read_register(Register::A));
}

// LDH [r8] A
pub fn ldh_r8_a(cpu: &mut Processor, instruction: u16) {
    let address_a = (instruction >> 8) as u8;
    let memory_address = to_u16(0xFF, address_a);

    cpu.write_memory(memory_address, cpu.read_register(Register::A));
}

// LDH A [r8]
pub fn ldh_a_r8(cpu: &mut Processor, instruction: u16) {
    let address_a = (instruction >> 8) as u8;
    let memory_address = to_u16(0xFF, address_a);
    let value = cpu.read_memory(memory_address);
    cpu.write_register(Register::A, value);
}

// LDH [C] A
pub fn ldh_c_a(cpu: &mut Processor, _instruction: u16) {
    let address_a = cpu.read_register(Register::C);
    let memory_address = to_u16(0xFF, address_a);

    cpu.write_memory(memory_address, cpu.read_register(Register::A));
}

// LDH A [C]
pub fn ldh_a_c(cpu: &mut Processor, _instruction: u16) {
    let address_a = cpu.read_register(Register::C);
    let memory_address = to_u16(0xFF, address_a);
    let value = cpu.read_memory(memory_address);

    cpu.write_register(Register::A, value);
}

/// RLCA
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

/// RRCA
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


#[cfg(test)]
mod test {
    use super::*;

    const ALL_REGISTERS: [Register; 8] = [
        Register::A,
        Register::F,
        Register::B,
        Register::C,
        Register::D,
        Register::E,
        Register::H,
        Register::L
    ];

    #[test]
    fn nop_works() {
        let mut cpu = Processor::new();
        nop(&mut cpu, 0);

        assert_eq!(0, 0);
    }

    #[test]
    fn increase_register_works() {
        let mut cpu = Processor::new();
        for register in ALL_REGISTERS {
            if register == Register::F { continue; }
            cpu.write_register(register, 0x00);
            increase_register(&mut cpu, 0x0000, register);

            assert_eq!(cpu.read_register(register), 0x01);
        }
    }

    #[test]
    fn increase_hl_pointer_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::H, 0x01);
        cpu.write_register(Register::L, 0x00);
        cpu.write_memory(0x0100, 0x00);

        inc_hlp(&mut cpu, 0x0000);

        assert_eq!(cpu.read_memory(0x0100), 0x1);
    }


    #[test]
    fn decrease_register_works() {
        let mut cpu = Processor::new();
        for register in ALL_REGISTERS {
            if register == Register::F { continue; }
            cpu.write_register(register, 0x02);
            decrease_register(&mut cpu, 0x0000, register);

            assert_eq!(cpu.read_register(register), 0x01);
        }
    }

    #[test]
    fn decrease_hl_pointer_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::L, 0x01);
        cpu.write_register(Register::H, 0x00);
        cpu.write_memory(0x0100, 0x02);

        dec_hlp(&mut cpu, 0x0000);

        assert_eq!(cpu.read_memory(0x0100), 0x1);
    }

    #[test]
    fn increase_double_register_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::B, 0x00);
        cpu.write_register(Register::C, 0xFF);
        cpu.write_register(Register::D, 0x00);
        cpu.write_register(Register::E, 0xFF);
        cpu.write_register(Register::H, 0x00);
        cpu.write_register(Register::L, 0xFF);
        cpu.stack_pointer = 0x00FF;

        increase_double_register(&mut cpu, 0x0000, Register::B, Register::C);
        increase_double_register(&mut cpu, 0x0000, Register::D, Register::E);
        increase_double_register(&mut cpu, 0x0000, Register::H, Register::L);
        inc_sp(&mut cpu, 0x0000);

        assert_eq!(cpu.read_register(Register::B), 0x01);
        assert_eq!(cpu.read_register(Register::D), 0x01);
        assert_eq!(cpu.read_register(Register::H), 0x01);
        assert_eq!(cpu.stack_pointer, 0x0100);
    }

    #[test]
    fn decrease_double_register_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::B, 0x01);
        cpu.write_register(Register::C, 0x00);
        cpu.write_register(Register::D, 0x01);
        cpu.write_register(Register::E, 0x00);
        cpu.write_register(Register::H, 0x01);
        cpu.write_register(Register::L, 0x00);
        cpu.stack_pointer = 0x0100;


        decrease_double_register(&mut cpu, 0x0000, Register::B, Register::C);
        decrease_double_register(&mut cpu, 0x0000, Register::D, Register::E);
        decrease_double_register(&mut cpu, 0x0000, Register::H, Register::L);
        dec_sp(&mut cpu, 0x0000);

        assert_eq!(cpu.read_register(Register::C), 0xFF);
        assert_eq!(cpu.read_register(Register::E), 0xFF);
        assert_eq!(cpu.read_register(Register::L), 0xFF);
        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn load_register_register_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::A, 0xAB);

        load_register_register(&mut cpu, 0x0000, Register::B, Register::A);
        load_register_register(&mut cpu, 0x0000, Register::C, Register::B);
        load_register_register(&mut cpu, 0x0000, Register::D, Register::C);
        load_register_register(&mut cpu, 0x0000, Register::E, Register::D);
        load_register_register(&mut cpu, 0x0000, Register::H, Register::E);
        load_register_register(&mut cpu, 0x0000, Register::L, Register::H);

        assert_eq!(cpu.read_register(Register::L), 0xAB);
    }

    #[test]
    fn load_register_value_works() {
        let mut cpu = Processor::new();

        load_register_value(&mut cpu, 0xAB00, Register::B);

        assert_eq!(cpu.read_register(Register::B), 0xAB);
    }

    #[test]
    fn load_double_register_value_works() {
        let mut cpu = Processor::new();

        load_double_register_value(&mut cpu, 0xCDAB, Register::B, Register::C);

        assert_eq!(cpu.read_register(Register::B), 0xAB);
        assert_eq!(cpu.read_register(Register::C), 0xCD);
    }

    #[test]
    fn load_stackpointer_value_works() {
        let mut cpu = Processor::new();

        ld_sp_n16(&mut cpu, 0xCDAB);

        assert_eq!(cpu.stack_pointer, 0xABCD);
    }

    #[test]
    fn load_hlp_register_works() {
        let mut cpu = Processor::new();

        cpu.write_register(Register::A, 0xAB);

        load_hlp_register(&mut cpu, 0x0000, Register::A);

        assert_eq!(cpu.read_register(Register::A), 0xAB);
    }

    #[test]
    fn load_hl_value_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::H, 0x01);
        cpu.write_register(Register::L, 0x00);

        ld_hlp_n8(&mut cpu, 0xAB00);

        assert_eq!(cpu.read_memory(0x0100), 0xAB);
    }

    #[test]
    fn load_register_hlp_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::H, 0x01);
        cpu.write_register(Register::L, 0x00);

        cpu.write_memory(0x0100, 0xAB);

        load_register_hlp(&mut cpu, 0x0000, Register::B);

        assert_eq!(cpu.read_register(Register::B), 0xAB);
    }

    #[test]
    fn load_double_registerp_a_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::B, 0x01);
        cpu.write_register(Register::C, 0x00);
        cpu.write_register(Register::A, 0xAB);

        load_double_registerp_a(&mut cpu, 0x0000, Register::B, Register::C);

        assert_eq!(cpu.read_memory(0x0100), 0xAB);
    }

    #[test]
    fn load_hli_a_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::H, 0x00);
        cpu.write_register(Register::L, 0xFF);
        cpu.write_register(Register::A, 0xAB);

        ld_hli_a(&mut cpu, 0x0000);

        assert_eq!(cpu.read_memory(0x00FF), 0xAB);
        assert_eq!(cpu.read_register(Register::H), 0x01);
    }

    #[test]
    fn load_hld_a_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::H, 0x01);
        cpu.write_register(Register::L, 0x00);
        cpu.write_register(Register::A, 0xAB);

        ld_hld_a(&mut cpu, 0x0000);

        assert_eq!(cpu.read_memory(0x0100), 0xAB);
        assert_eq!(cpu.read_register(Register::L), 0xFF);
    }

    #[test]
    fn load_double_value_a_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::A, 0xAB);

        ld_r16_a(&mut cpu, 0x0001);

        assert_eq!(cpu.read_memory(0x0100), 0xAB);
    }

    #[test]
    fn loadh_value_a_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::A, 0xAB);

        ldh_r8_a(&mut cpu, 0x0100);

        assert_eq!(cpu.read_memory(0xFF01), 0xAB);
    }

    #[test]
    fn loadh_a_value_works() {
        let mut cpu = Processor::new();
        cpu.write_memory(0xFF01, 0xAB);

        ldh_a_r8(&mut cpu, 0x0100);

        assert_eq!(cpu.read_register(Register::A), 0xAB);
    }

    #[test]
    fn loadh_c_a_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::A, 0xAB);
        cpu.write_register(Register::C, 0x01);

        ldh_c_a(&mut cpu, 0x0000);

        assert_eq!(cpu.read_memory(0xFF01), 0xAB);
    }
    
    #[test]
    fn loadh_a_c_works() {
        let mut cpu = Processor::new();
        cpu.write_register(Register::C, 0x01);
        cpu.write_memory(0xFF01, 0xAB);

        ldh_a_c(&mut cpu, 0x0000);

        assert_eq!(cpu.read_register(Register::A), 0xAB);
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
}
