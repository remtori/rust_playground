use std::{collections::HashMap, ops::Range};

use crate::{cpu6502::Cpu6502, system::SystemBus, Device};

#[derive(Debug, Default)]
pub struct Emulator {
    cpu: Cpu6502,
    system_bus: SystemBus,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            cpu: Cpu6502::new(),
            system_bus: SystemBus::new(),
        }
    }

    pub fn tick(&mut self, cycle: u16) {
        let bus = &mut self.system_bus;
        for _ in 0..cycle {
            self.cpu.tick(bus);
        }
    }

    pub fn disassemble(&mut self, addr_range: Range<u16>) -> HashMap<u16, String> {
        let bus = &mut self.system_bus;
        self.cpu.disassemble(bus, addr_range)
    }

    pub fn cpu(&self) -> &Cpu6502 {
        &self.cpu
    }

    pub fn reset(&mut self) {
        let bus = &mut self.system_bus;
        self.cpu.reset(bus);
    }

    pub fn step(&mut self) {
        let bus = &mut self.system_bus;
        loop {
            self.cpu.tick(bus);
            if self.cpu.complete() {
                break;
            }
        }
    }

    pub fn write_ram(&mut self, offset: u16, data: &[u8]) {
        let base_offset = offset as usize;
        for (offset, byte) in data.iter().enumerate() {
            self.system_bus.ram[offset + base_offset] = *byte;
        }
    }
}
