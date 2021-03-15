#![allow(clippy::identity_op)]
#![allow(clippy::upper_case_acronyms)]

pub mod cartridge;
pub mod cpu6502;
pub mod emulator;
pub mod mapper;
pub mod ppu2C02;
pub mod system;

pub type Bus<'a> = &'a mut crate::system::SystemBus;

pub trait Device {
    fn tick(&mut self, bus: &mut crate::system::SystemBus);
}
