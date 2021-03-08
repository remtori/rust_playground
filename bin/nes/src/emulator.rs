use std::{collections::HashMap, ops::Range};

use crate::{cpu6502::Cpu6502, Device};

pub struct Emulator {
    cpu: Cpu6502,
    pub ram: [u8; 64 * 1024],
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            cpu: Cpu6502::new(),
            ram: [0u8; 64 * 1024],
        }
    }

    pub fn tick(&mut self, cycle: u16) {
        let bus = self.bus();
        for _ in 0..cycle {
            self.cpu.tick(bus);
        }
    }

    pub fn disassemble(&mut self, addr_range: Range<u16>) -> HashMap<u16, String> {
        let bus = self.bus();
        self.cpu.disassemble(bus, addr_range)
    }

    pub fn cpu(&self) -> &Cpu6502 {
        &self.cpu
    }

    pub fn reset(&mut self) {
        let bus = self.bus();
        self.cpu.reset(bus);
    }

    pub fn step(&mut self) {
        let bus = self.bus();
        loop {
            self.cpu.tick(bus);
            if self.cpu.complete() {
                break;
            }
        }
    }

    pub fn bus(&mut self) -> &'static mut dyn crate::Bus {
        // Hacky work around to satisfy the borrow checker
        // in the implementation of Bus trait we never mutate the CPU
        // so this is safe
        let bus: *mut dyn crate::Bus = self;
        unsafe { &mut *bus }
    }
}

impl Default for Emulator {
    fn default() -> Self {
        Emulator::new()
    }
}

impl crate::Bus for Emulator {
    fn write(&mut self, addr: u16, data: u8) {
        if (0x0000..=0xffff).contains(&addr) {
            self.ram[addr as usize] = data;
        }
    }

    fn read(&self, addr: u16, _readonly: bool) -> u8 {
        if (0x0000..=0xffff).contains(&addr) {
            self.ram[addr as usize]
        } else {
            0
        }
    }
}
