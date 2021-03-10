#[derive(Debug)]
pub struct SystemBus {
    pub(crate) ram: [u8; 2 * 1024],
    pub(crate) nameTable: [[u8; 1024]; 2],
    pub(crate) paletteTable: [u8; 32],
    pub(crate) patternTable: [u8; 4096],
}

impl SystemBus {
    pub fn new() -> SystemBus {
        SystemBus {
            ram: [0u8; 2 * 1024],
            nameTable: [[0u8; 1024]; 2],
            paletteTable: [0u8; 32],
            patternTable: [0u8; 4096],
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        if (0x0000..=0x1fff).contains(&addr) {
            self.ram[(addr & 0x07FF) as usize] = data;
        } else if (0x2000..=0x3FFF).contains(&addr) {
            // ppu.cpuWrite(addr & 0x0007, data);
        }
    }

    pub fn read(&self, addr: u16, _readonly: bool) -> u8 {
        if (0x0000..=0x1fff).contains(&addr) {
            self.ram[(addr & 0x07FF) as usize]
        } else if (0x2000..=0x3FFF).contains(&addr) {
            // ppu.cpuRead(addr & 0x0007, readonly)
            1
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
