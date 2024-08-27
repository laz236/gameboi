use crate::memory_map::IMemoryMap;

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
