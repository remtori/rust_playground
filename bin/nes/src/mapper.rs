#[derive(Debug)]
pub struct Mapper {
    prg_banks: u8,
    chr_banks: u8,
    imp: MapperImpl,
}

impl Mapper {
    pub fn new(mapper_id: u8, prg_banks: u8, chr_banks: u8) -> Mapper {
        Mapper {
            prg_banks,
            chr_banks,
            imp: create_mapper_impl(mapper_id),
        }
    }

    #[inline]
    pub fn cpu_map_read(&self, addr: u16) -> Option<u16> {
        (self.imp.cpu_map_read)(addr, self.prg_banks, self.chr_banks)
    }

    #[inline]
    pub fn cpu_map_write(&self, addr: u16) -> Option<u16> {
        (self.imp.cpu_map_write)(addr, self.prg_banks, self.chr_banks)
    }

    #[inline]
    pub fn ppu_map_read(&self, addr: u16) -> Option<u16> {
        (self.imp.ppu_map_read)(addr, self.prg_banks, self.chr_banks)
    }

    #[inline]
    pub fn ppu_map_write(&self, addr: u16) -> Option<u16> {
        (self.imp.ppu_map_write)(addr, self.prg_banks, self.chr_banks)
    }
}

#[derive(Debug)]
struct MapperImpl {
    cpu_map_read: fn(addr: u16, prg_banks: u8, chr_banks: u8) -> Option<u16>,
    cpu_map_write: fn(addr: u16, prg_banks: u8, chr_banks: u8) -> Option<u16>,
    ppu_map_read: fn(addr: u16, prg_banks: u8, chr_banks: u8) -> Option<u16>,
    ppu_map_write: fn(addr: u16, prg_banks: u8, chr_banks: u8) -> Option<u16>,
}

fn create_mapper_impl(mapper_id: u8) -> MapperImpl {
    match mapper_id {
        0 => MapperImpl {
            // if PRGROM is 16KB
            //     CPU Address Bus          PRG ROM
            //     0x8000 -> 0xBFFF: Map    0x0000 -> 0x3FFF
            //     0xC000 -> 0xFFFF: Mirror 0x0000 -> 0x3FFF
            // if PRGROM is 32KB
            //     CPU Address Bus          PRG ROM
            //     0x8000 -> 0xFFFF: Map    0x0000 -> 0x7FFF
            cpu_map_read: |addr, prg_banks, _| {
                if (0x8000..=0xFFFF).contains(&addr) {
                    Some(addr & if prg_banks > 1 { 0x7FFF } else { 0x3FFF })
                } else {
                    None
                }
            },
            cpu_map_write: |addr, prg_banks, _| {
                if (0x8000..=0xFFFF).contains(&addr) {
                    Some(addr & if prg_banks > 1 { 0x7FFF } else { 0x3FFF })
                } else {
                    None
                }
            },
            // There is no mapping required for PPU
            // PPU Address Bus          CHR ROM
            // 0x0000 -> 0x1FFF: Map    0x0000 -> 0x1FFF
            ppu_map_read: |addr, _, _| {
                if (0x0000..=0x1FFF).contains(&addr) {
                    Some(addr)
                } else {
                    None
                }
            },
            ppu_map_write: |addr, _, _| {
                if (0x0000..=0x1FFF).contains(&addr) {
                    Some(addr)
                } else {
                    None
                }
            },
        },
        _ => unimplemented!(),
    }
}
