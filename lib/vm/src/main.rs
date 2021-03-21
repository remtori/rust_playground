use memmap2::MmapMut;
use std::io::Write;

use std::*;

const PROGRAM: &[u8] = &[0xb8, 0x01, 0x00, 0x00, 0x00, 0xc3];

fn main() -> Result<(), std::io::Error> {
    let mut mmap = MmapMut::map_anon(32)?;

    (&mut mmap[..]).write_all(PROGRAM)?;
    println!("Write");

    let exec_map = mmap.make_exec()?;
    println!("Make exec");

    let func: extern "C" fn() -> bool = unsafe { mem::transmute(exec_map.as_ptr()) };
    println!("Cast");

    println!("ret={}", func());

    Ok(())
}
