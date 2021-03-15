use crate::{cartridge::Cartridge, ppu2C02::Ppu2C02};

#[derive(Debug)]
pub struct SystemBus {
    pub(crate) ram: [u8; 2 * 1024],
    pub(crate) ppu: Ppu2C02,
    cartridge: Option<Cartridge>,
}

impl SystemBus {
    pub fn new() -> SystemBus {
        SystemBus {
            ram: [0u8; 2 * 1024],
            ppu: Ppu2C02::new(),
            cartridge: None,
        }
    }

    pub fn insert_cartridge(&mut self, cartridge: Cartridge) {
        self.cartridge = Some(cartridge)
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        let cart = self.cartridge.as_mut().unwrap();

        if cart.cpu_write(addr, data) {
        } else if (0x0000..=0x1fff).contains(&addr) {
            self.ram[(addr & 0x07FF) as usize] = data;
        } else if (0x2000..=0x3FFF).contains(&addr) {
            self.ppu.cpu_write(addr & 0x0007, data);
        }
    }

    pub fn read(&mut self, addr: u16, readonly: bool) -> u8 {
        let cart = self.cartridge.as_mut().unwrap();

        if let Some(data) = cart.cpu_read(addr) {
            data
        } else if (0x0000..=0x1fff).contains(&addr) {
            self.ram[(addr & 0x07FF) as usize]
        } else if (0x2000..=0x3FFF).contains(&addr) {
            self.ppu.cpu_read(addr & 0x0007, readonly)
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
