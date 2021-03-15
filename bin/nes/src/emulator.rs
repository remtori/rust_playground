use std::{collections::HashMap, ops::Range};

use crate::{cpu6502::Cpu6502, ppu2C02::Ppu2C02, system::SystemBus, Device};

#[derive(Debug, Default)]
pub struct Emulator {
    cpu: Cpu6502,
    system_bus: SystemBus,
    clock_counter: u32,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator {
            cpu: Cpu6502::new(),
            system_bus: SystemBus::new(),
            clock_counter: 0,
        }
    }

    pub fn tick(&mut self) {
        let bus = &mut self.system_bus;

        bus.ppu.tick();
        if self.clock_counter % 3 == 0 {
            self.cpu.tick(bus);
        }

        self.clock_counter += 1;
    }

    pub fn disassemble(&mut self, addr_range: Range<u16>) -> HashMap<u16, String> {
        let bus = &mut self.system_bus;
        self.cpu.disassemble(bus, addr_range)
    }

    pub fn cpu(&self) -> &Cpu6502 {
        &self.cpu
    }

    pub fn ppu(&self) -> &Ppu2C02 {
        &self.system_bus.ppu
    }

    pub fn reset(&mut self) {
        let bus = &mut self.system_bus;
        self.cpu.reset(bus);
        self.clock_counter = 0;
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
}
