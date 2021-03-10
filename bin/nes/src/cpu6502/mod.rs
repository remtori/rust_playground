pub mod addressing_mode;
pub mod disassemble;
pub mod instruction;
pub mod lookup;

use crate::*;
use lookup::{lookup_instruction, InsnFunc};

pub enum Flags {
    /// Carry Bit
    C = 1 << 0,

    /// Zero
    Z = 1 << 1,

    /// Disable Interupts
    I = 1 << 2,

    /// Decimal Mode
    D = 1 << 3,

    /// Break
    B = 1 << 4,

    /// Unused
    U = 1 << 5,

    /// Overflow
    V = 1 << 6,

    /// Negative
    N = 1 << 7,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cpu6502 {
    /// Accumulator Register
    a: u8,

    /// X Register
    x: u8,

    /// Y Register
    y: u8,

    /// Stack Pointer
    stkp: u8,

    /// Program Counter
    pc: u16,

    /// Status Register
    status: u8,

    addr_abs: u16,
    addr_rel: u16,
    opcode: u8,
    cycles: u8,
    fetched: u8,
}

impl Cpu6502 {
    pub fn new() -> Cpu6502 {
        Cpu6502 {
            a: 0,
            x: 0,
            y: 0,
            stkp: 0,
            pc: 0,
            status: 0,
            addr_abs: 0,
            addr_rel: 0,
            opcode: 0,
            cycles: 0,
            fetched: 0,
        }
    }

    pub fn register_a(&self) -> u8 {
        self.a
    }

    pub fn register_x(&self) -> u8 {
        self.x
    }

    pub fn register_y(&self) -> u8 {
        self.y
    }

    pub fn stack_pointer(&self) -> u8 {
        self.stkp
    }

    pub fn program_counter(&self) -> u16 {
        self.pc
    }

    pub fn complete(&self) -> bool {
        self.cycles == 0
    }

    pub const BASE_STACK_PTR: u16 = 0x0100;
    pub const NON_MASKABLE_INTERUPT_PC: u16 = 0xFFFA;
    pub const INTERUPT_PC: u16 = 0xFFFE;
    pub const DEFAULT_PC: u16 = 0xFFFC;

    pub fn flag(&self, flag: Flags) -> u8 {
        if self.status & flag as u8 > 0 {
            1
        } else {
            0
        }
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        if value {
            self.status |= flag as u8;
        } else {
            self.status &= !(flag as u8);
        }
    }

    pub fn set_zero_negative_flag(&mut self, value: u8) {
        self.set_flag(Flags::Z, value == 0);
        self.set_flag(Flags::N, value & 0x80 > 0);
    }
}

impl crate::Device for Cpu6502 {
    fn tick(&mut self, bus: Bus) {
        if self.cycles == 0 {
            self.opcode = bus.read(self.pc, false);
            self.pc += 1;

            let insn = lookup_instruction(self.opcode);
            self.cycles = insn.cycles;

            let addr_need_more_cycles = (insn.addr_mode)(self, bus);
            let oper_need_more_cycles = (insn.operate)(self, bus);
            self.cycles += addr_need_more_cycles & oper_need_more_cycles;
        }

        self.cycles -= 1;
    }
}

impl Cpu6502 {
    pub fn reset(&mut self, bus: Bus) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.stkp = 0xFD;
        self.status = 0x00 | Flags::U as u8;

        self.addr_abs = Self::DEFAULT_PC;
        let lo = bus.read(self.addr_abs + 0, false) as u16;
        let hi = bus.read(self.addr_abs + 1, false) as u16;
        self.pc = (hi << 8) | lo;

        self.addr_rel = 0;
        self.addr_abs = 0;
        self.fetched = 0;

        self.cycles = 8;
    }

    pub fn interrupt_requested(&mut self, bus: Bus) {
        // Interupt is disabled
        if self.flag(Flags::I) == 1 {
            return;
        }

        // Store program counter, which is u16 so its take 2 write
        bus.write(
            Self::BASE_STACK_PTR + self.stkp as u16,
            ((self.pc >> 8) & 0x00FF) as u8,
        );
        self.stkp -= 1;

        bus.write(
            Self::BASE_STACK_PTR + self.stkp as u16,
            (self.pc & 0x00FF) as u8,
        );
        self.stkp -= 1;

        self.set_flag(Flags::B, false);
        self.set_flag(Flags::U, true);
        self.set_flag(Flags::I, true);

        // Store status register
        bus.write(Self::BASE_STACK_PTR + self.stkp as u16, self.status);
        self.stkp -= 1;

        self.addr_abs = Self::INTERUPT_PC;
        let lo = bus.read(self.addr_abs + 0, false) as u16;
        let hi = bus.read(self.addr_abs + 1, false) as u16;
        self.pc = (hi << 8) | lo;

        self.cycles = 7;
    }

    pub fn non_maskable_interrupt(&mut self, bus: Bus) {
        // Store program counter, which is u16 so its take 2 write
        bus.write(
            Self::BASE_STACK_PTR + self.stkp as u16,
            ((self.pc >> 8) & 0x00FF) as u8,
        );
        self.stkp -= 1;

        bus.write(
            Self::BASE_STACK_PTR + self.stkp as u16,
            (self.pc & 0x00FF) as u8,
        );
        self.stkp -= 1;

        self.set_flag(Flags::B, false);
        self.set_flag(Flags::U, true);
        self.set_flag(Flags::I, true);

        // Store status register
        bus.write(Self::BASE_STACK_PTR + self.stkp as u16, self.status);
        self.stkp -= 1;

        self.addr_abs = Self::NON_MASKABLE_INTERUPT_PC;
        let lo = bus.read(self.addr_abs + 0, false) as u16;
        let hi = bus.read(self.addr_abs + 1, false) as u16;
        self.pc = (hi << 8) | lo;

        self.cycles = 7;
    }
}

/// Pointer comparison to check if two addressing mode is the same
#[inline]
pub fn is_same_addr_mode(a: InsnFunc, b: InsnFunc) -> bool {
    (a as *const InsnFunc).eq(&(b as *const InsnFunc))
}
