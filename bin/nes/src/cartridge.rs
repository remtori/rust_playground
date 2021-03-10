use std::{ffi::OsString, fs::File, io::*};

#[derive(Debug, Default)]
pub struct Cartridge {
    prg_mem: Vec<u8>,
    chr_mem: Vec<u8>,
    mapper_id: u8,
    prg_bank: u8,
    chr_bank: u8,
}

#[repr(C)]
#[derive(Debug, Default)]
struct Header {
    name: [char; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: [char; 4],
}

impl Cartridge {
    pub fn from_file(file_path: OsString) -> Result<Cartridge> {
        let mut cartridge = Cartridge::default();
        let mut file = File::open(file_path)?;

        let mut header = Header::default();
        let header_slice = unsafe {
            std::slice::from_raw_parts_mut(
                &mut header as *mut _ as *mut u8,
                std::mem::size_of::<Header>(),
            )
        };

        file.read_exact(header_slice)?;
        if header.mapper1 & 0x04 > 0 {
            file.seek(SeekFrom::Current(512))?;
        }

        cartridge.mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1) >> 4;

        let file_type = 1;

        match file_type {
            0 => {}
            1 => {
                cartridge.prg_bank = header.prg_rom_chunks;
                cartridge
                    .prg_mem
                    .reserve_exact(cartridge.prg_bank as usize * 16384);

                file.read_exact(cartridge.prg_mem.as_mut_slice())?;

                cartridge.chr_bank = header.chr_rom_chunks;
                cartridge
                    .chr_mem
                    .reserve_exact(cartridge.chr_bank as usize * 8192);

                file.read_exact(cartridge.chr_mem.as_mut_slice())?;
            }
            2 => {}
            _ => unreachable!(),
        }

        Ok(cartridge)
    }
}
