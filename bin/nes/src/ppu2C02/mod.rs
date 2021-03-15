mod palette;
use palette::PALETTE;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel(u8, u8, u8, u8);

const BLACK: Pixel = Pixel(0, 0, 0, 255);

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;
pub const PATTERN_TABLE_SIZE: usize = 128;

#[derive(Debug)]
pub struct Ppu2C02 {
    pub(crate) name_table: [[u8; 1024]; 2],
    pub(crate) palette_table: [u8; 32],
    pub(crate) pattern_table: [u8; 4096],
    cycle: i16,
    scanline: i16,
    frame_complete: bool,

    rendered_screen: Vec<Pixel>,
    rendered_name_table: [Vec<Pixel>; 2],
    rendered_pattern_table: [Vec<Pixel>; 2],
}

impl Ppu2C02 {
    pub fn new() -> Ppu2C02 {
        Ppu2C02 {
            name_table: [[0u8; 1024]; 2],
            palette_table: [0u8; 32],
            pattern_table: [0u8; 4096],
            cycle: 0,
            scanline: 0,
            frame_complete: false,
            rendered_screen: vec![BLACK; SCREEN_WIDTH * SCREEN_HEIGHT],
            rendered_name_table: [
                vec![BLACK; SCREEN_WIDTH * SCREEN_HEIGHT],
                vec![BLACK; SCREEN_WIDTH * SCREEN_HEIGHT],
            ],
            rendered_pattern_table: [
                vec![BLACK; PATTERN_TABLE_SIZE * PATTERN_TABLE_SIZE],
                vec![BLACK; PATTERN_TABLE_SIZE * PATTERN_TABLE_SIZE],
            ],
        }
    }

    pub fn screen(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.rendered_screen.as_ptr() as *const u8,
                self.rendered_screen.len() * std::mem::size_of::<Pixel>(),
            )
        }
    }

    pub fn name_table(&self, index: u8) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.rendered_name_table[index as usize].as_ptr() as *const u8,
                self.rendered_name_table[index as usize].len() * std::mem::size_of::<Pixel>(),
            )
        }
    }

    pub fn pattern_table(&mut self, index: u8, palette: u8) -> &[u8] {
        let index = index as u16;

        for tile_y in 0..16 {
            for tile_x in 0..16 {
                let offset = tile_y * 256 + tile_x * 16;

                for row in 0..8 {
                    let mut tile_lsb = self.ppu_read(index * 0x1000 + offset + row + 0, false);
                    let mut tile_msb = self.ppu_read(index * 0x1000 + offset + row + 8, false);

                    for col in 0..8 {
                        let pixel = (tile_lsb & 0x01) + (tile_msb & 0x01);
                        tile_lsb >>= 1;
                        tile_msb >>= 1;

                        self.rendered_pattern_table[index as usize][
                            // x axis
                            (tile_x * 8 + (7 - col)) as usize
                            // y axis
                            + (tile_y * 8 + row) as usize
                            * PATTERN_TABLE_SIZE
                        ] = self.color_from_palette_ram(palette, pixel)
                    }
                }
            }
        }

        unsafe {
            core::slice::from_raw_parts(
                self.rendered_pattern_table[index as usize].as_ptr() as *const u8,
                self.rendered_pattern_table[index as usize].len() * std::mem::size_of::<Pixel>(),
            )
        }
    }

    fn color_from_palette_ram(&mut self, palette: u8, pixel: u8) -> Pixel {
        PALETTE[(self.ppu_read(0x3F00 + ((palette as u16) << 2) + pixel as u16, false) & 0x3F)
            as usize]
    }

    pub fn tick(&mut self) {
        self.cycle += 1;
        if self.cycle >= 341 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline >= 261 {
                self.scanline = -1;
                self.frame_complete = true;
            }
        }
    }

    pub fn cpu_read(&mut self, addr: u16, readonly: bool) -> u8 {
        match addr {
            // Control
            0x0000 => todo!(),

            // Mask
            0x0001 => todo!(),

            // Status
            0x0002 => todo!(),

            // OAM Address
            0x0003 => todo!(),

            // OAM Data
            0x0004 => todo!(),

            // Scroll
            0x0005 => todo!(),

            // PPU Address
            0x0006 => todo!(),

            // PPU Data
            0x0007 => todo!(),

            _ => unreachable!(),
        }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr {
            // Control
            0x0000 => todo!(),

            // Mask
            0x0001 => todo!(),

            // Status
            0x0002 => todo!(),

            // OAM Address
            0x0003 => todo!(),

            // OAM Data
            0x0004 => todo!(),

            // Scroll
            0x0005 => todo!(),

            // PPU Address
            0x0006 => todo!(),

            // PPU Data
            0x0007 => todo!(),

            _ => unreachable!(),
        }
    }

    pub fn ppu_read(&mut self, addr: u16, readonly: bool) -> u8 {
        let mut data = 0u8;
        let addr = addr & 0x3FFF;
        data
    }

    pub fn ppu_write(&mut self, addr: u16, data: u8) {
        let addr = addr & 0x3FFF;
    }
}

impl Default for Ppu2C02 {
    fn default() -> Self {
        Self::new()
    }
}
