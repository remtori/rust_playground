use std::{collections::HashMap, ops::Range};

use crate::{cpu6502::Cpu6502, Device};

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

#[derive(Debug)]
pub struct SystemBus {
    ram: [u8; 64 * 1024],
}

impl SystemBus {
    pub fn new() -> SystemBus {
        SystemBus {
            ram: [0u8; 64 * 1024],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if (0x0000..=0xffff).contains(&addr) {
            self.ram[addr as usize] = data;
        }
    }

    pub fn read(&self, addr: u16, _readonly: bool) -> u8 {
        if (0x0000..=0xffff).contains(&addr) {
            self.ram[addr as usize]
        } else {
            0
        }
    }
}

impl Default for SystemBus {
    fn default() -> Self {
        Self::new()
    }
}
