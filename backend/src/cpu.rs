use crate::memory_map::IMemoryMap;

mod flag;
mod instruction;

pub struct Interrupt {
    // ime
    interrupt_master_enable: bool,
}

pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

pub struct Cpu {
    mmu: Box<dyn IMemoryMap>,
    reg: Registers,
    interrupt: Interrupt,
    pc: u16,
    sp: u16,
}

impl Cpu {
    pub fn new(registers: Registers, memory_map: Box<dyn IMemoryMap>) -> Cpu {
        Cpu {
            mmu: memory_map,
            reg: registers,
            interrupt: Interrupt {
                interrupt_master_enable: false,
            },
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }
}

impl Cpu {
    pub fn fetch_decode_execute(&mut self) {}
}
