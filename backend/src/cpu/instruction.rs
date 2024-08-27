use crate::{
    bitwise,
    cpu::flag,
    cpu::flag::{CARRY, HALF_CARRY, SUBTRACTION, ZERO},
    memory_map::IMemoryMap,
};

fn _stack_push<T: IMemoryMap>(bus: &mut T, stack_pointer: &mut u16, data: u8) {
    *stack_pointer = stack_pointer.wrapping_sub(1);

    bus.write(*stack_pointer, data);
}

fn _stack_pop<T: IMemoryMap>(bus: &mut T, stack_pointer: &mut u16) -> u8 {
    let result = bus.read(*stack_pointer);

    *stack_pointer = stack_pointer.wrapping_add(1);

    result
}

pub fn pop_pair<T: IMemoryMap>(bus: &mut T, stack_pointer: &mut u16, hi: &mut u8, lo: &mut u8) {
    *lo = _stack_pop(bus, stack_pointer);
    *hi = _stack_pop(bus, stack_pointer);
}

pub fn push_pair<T: IMemoryMap>(bus: &mut T, stack_pointer: &mut u16, hi: u8, lo: u8) {
    _stack_push(bus, stack_pointer, hi);
    _stack_push(bus, stack_pointer, lo);
}

pub fn rotate_left(flags: &mut u8, register: &mut u8) {
    let old_carry = bitwise::test_bit(*register, CARRY);
    let new_carry = bitwise::test_bit(*register, 1 << 7);

    *register <<= 1;

    if old_carry {
        *register |= 1;
    }

    flag::apply_zero(flags, *register);
    bitwise::set_bit_off(flags, SUBTRACTION | HALF_CARRY);
    bitwise::set_bit_dynamic(flags, CARRY, new_carry);
}

pub fn rotate_left_through_carry(flags: &mut u8, register: &mut u8) {
    let new_carry = bitwise::test_bit(*register, 1 << 7);

    *register <<= 1;

    if new_carry {
        *register |= 1;
    }

    flag::apply_zero(flags, *register);
    bitwise::set_bit_off(flags, SUBTRACTION | HALF_CARRY);
    bitwise::set_bit_dynamic(flags, CARRY, new_carry);
}

pub fn shift_left_arithmetically(flags: &mut u8, register: &mut u8) {
    let new_carry = bitwise::test_bit(*register, 1 << 7);

    *register <<= 1;

    flag::apply_zero(flags, *register);
    bitwise::set_bit_off(flags, SUBTRACTION | HALF_CARRY);
    bitwise::set_bit_dynamic(flags, CARRY, new_carry);
}

pub fn rotate_right(flags: &mut u8, register: &mut u8) {
    let old_carry = bitwise::test_bit(*register, CARRY);
    let new_carry = bitwise::test_bit(*register, 1);

    *register >>= 1;

    if old_carry {
        *register |= 1 << 7;
    }

    flag::apply_zero(flags, *register);
    bitwise::set_bit_off(flags, SUBTRACTION | HALF_CARRY);
    bitwise::set_bit_dynamic(flags, CARRY, new_carry);
}

pub fn rotate_right_through_carry(flags: &mut u8, register: &mut u8) {
    let new_carry = bitwise::test_bit(*register, 1);

    *register >>= 1;

    if new_carry {
        *register |= 1 << 7;
    }

    flag::apply_zero(flags, *register);
    bitwise::set_bit_off(flags, SUBTRACTION | HALF_CARRY);
    bitwise::set_bit_dynamic(flags, CARRY, new_carry);
}

pub fn shift_right_arithmetically(flags: &mut u8, register: &mut u8) {
    let new_carry = bitwise::test_bit(*register, 1);

    *register = (*register >> 1) | (*register & 1 << 7);

    flag::apply_zero(flags, *register);
    bitwise::set_bit_off(flags, SUBTRACTION | HALF_CARRY);
    bitwise::set_bit_dynamic(flags, CARRY, new_carry);
}

pub fn shift_right_logically(flags: &mut u8, register: &mut u8) {
    let new_carry = bitwise::test_bit(*register, 1);

    *register >>= 1;

    flag::apply_zero(flags, *register);
    bitwise::set_bit_off(flags, SUBTRACTION | HALF_CARRY);
    bitwise::set_bit_dynamic(flags, CARRY, new_carry);
}
