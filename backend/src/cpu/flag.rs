use crate::bitwise::set_bit_dynamic;

// Z
pub const ZERO: u8 = 1 << 7;
// N
pub const SUBTRACTION: u8 = 1 << 6;
// H
pub const HALF_CARRY: u8 = 1 << 5;
// C
pub const CARRY: u8 = 1 << 4;

pub fn apply_zero(register: &mut u8, data: u8) {
    set_bit_dynamic(register, ZERO, data == 0)
}

pub fn apply_carry_8(register: &mut u8, lhs: u8, rhs: u8, carry: u8) {
    set_bit_dynamic(
        register,
        CARRY,
        (lhs as u16) + (rhs as u16) + (carry as u16) > 0xFF,
    )
}

pub fn apply_carry_16(register: &mut u8, lhs: u16, rhs: u16, carry: u8) {
    set_bit_dynamic(
        register,
        CARRY,
        (lhs as u32) + (rhs as u32) + (carry as u32) > 0xFFFF,
    )
}

pub fn apply_borrow_8(register: &mut u8, lhs: u8, rhs: u8, carry: u8) {
    set_bit_dynamic(
        register,
        CARRY,
        (lhs & 0x0F) < (rhs & 0x0F) + (carry & 0x0F),
    )
}

pub fn apply_borrow_16(register: &mut u8, lhs: u16, rhs: u16, carry: u8) {
    set_bit_dynamic(
        register,
        CARRY,
        (lhs as u32) < (rhs as u32) + (carry as u32),
    )
}

pub fn apply_half_carry_8(register: &mut u8, lhs: u8, rhs: u8, carry: u8) {
    set_bit_dynamic(
        register,
        HALF_CARRY,
        (lhs & 0x0F) + (rhs & 0x0F) + (carry & 0x0F) > 0x0F,
    )
}

pub fn apply_half_carry_16(register: &mut u8, lhs: u16, rhs: u16, carry: u8) {
    set_bit_dynamic(
        register,
        HALF_CARRY,
        (lhs & 0x0FFF) + (rhs & 0x0FFF) + (carry as u16 & 0x0FFF) > 0x0FFF,
    )
}

pub fn apply_half_borrow_8(register: &mut u8, lhs: u8, rhs: u8, carry: u8) {
    set_bit_dynamic(
        register,
        HALF_CARRY,
        (lhs & 0x0F) < (rhs & 0x0F) + (carry & 0x0F),
    )
}

pub fn apply_half_borrow_16(register: &mut u8, lhs: u16, rhs: u16, carry: u8) {
    set_bit_dynamic(
        register,
        HALF_CARRY,
        (lhs & 0x0FFF) < (rhs & 0x0FFF) + (carry as u16 & 0x0FFF),
    )
}
