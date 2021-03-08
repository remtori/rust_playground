#![allow(clippy::identity_op)]

pub mod cpu6502;
pub mod emulator;

pub trait Bus {
    fn read(&self, addr: u16, readonly: bool) -> u8;
    fn write(&mut self, addr: u16, data: u8);
}

pub trait Device {
    fn tick(&mut self, bus: &mut dyn Bus);
}
