use crate::mapper::Mapper;
use std::{
    ffi::OsString,
    fs::File,
    io::{Read, Seek, SeekFrom},
};

#[derive(Debug)]
pub struct Cartridge {
    prg_mem: Vec<u8>,
    chr_mem: Vec<u8>,
    mapper_id: u8,
    prg_banks: u8,
    chr_banks: u8,
    mapper: Mapper,
}

#[repr(C)]
#[derive(Debug, Default)]
struct Header {
    name: [u8; 4],
    prg_rom_chunks: u8,
    chr_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prg_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: [u8; 4],
}

pub enum Error {
    IO(std::io::Error),
    InvalidData,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IO(e)
    }
}

impl Cartridge {
    pub fn from_file(file_path: OsString) -> Result<Cartridge, Error> {
        let mut file = File::open(file_path)?;

        let mut header = Header::default();
        let header_slice = unsafe {
            std::slice::from_raw_parts_mut(
                &mut header as *mut _ as *mut u8,
                std::mem::size_of::<Header>(),
            )
        };

        file.read_exact(header_slice)?;

        if !header.name.eq(b"NES\x1A") {
            return Err(Error::InvalidData);
        }

        if header.mapper1 & 0x04 > 0 {
            file.seek(SeekFrom::Current(512))?;
        }

        let mapper_id = ((header.mapper2 >> 4) << 4) | (header.mapper1) >> 4;

        let file_type = 1;

        let (prg_banks, chr_banks, prg_mem, chr_mem) = match file_type {
            0 => todo!(),
            1 => {
                let mut prg_mem = vec![0u8; header.prg_rom_chunks as usize * 16384];
                file.read_exact(prg_mem.as_mut_slice())?;

                let mut chr_mem = vec![0u8; header.chr_rom_chunks as usize * 8192];
                file.read_exact(chr_mem.as_mut_slice())?;

                (
                    header.prg_rom_chunks,
                    header.chr_rom_chunks,
                    prg_mem,
                    chr_mem,
                )
            }
            2 => todo!(),
            _ => unreachable!(),
        };

        Ok(Cartridge {
            prg_mem,
            chr_mem,
            mapper_id,
            prg_banks,
            chr_banks,
            mapper: Mapper::new(mapper_id, prg_banks, chr_banks),
        })
    }

    pub fn cpu_read(&mut self, addr: u16) -> Option<u8> {
        self.mapper
            .cpu_map_read(addr)
            .map(|mapped_addr| self.prg_mem[mapped_addr as usize])
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
        self.mapper
            .cpu_map_write(addr)
            .map(|mapped_addr| self.prg_mem[mapped_addr as usize] = data)
            .is_some()
    }

    pub fn ppu_read(&mut self, addr: u16) -> Option<u8> {
        self.mapper
            .ppu_map_read(addr)
            .map(|mapped_addr| self.chr_mem[mapped_addr as usize])
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) -> bool {
        self.mapper
            .ppu_map_write(addr)
            .map(|mapped_addr| self.chr_mem[mapped_addr as usize] = data)
            .is_some()
    }
}
