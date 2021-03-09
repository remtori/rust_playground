#![allow(clippy::identity_op)]

pub mod cpu6502;
pub mod emulator;

pub type Bus<'a> = &'a mut crate::emulator::SystemBus;

pub trait Device {
    fn tick(&mut self, bus: &mut crate::emulator::SystemBus);
}
