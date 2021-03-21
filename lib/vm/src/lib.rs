#![allow(clippy::upper_case_acronyms)]

/// Bit flag to the FLAGS register
pub enum Flag {
    /// Carry Flag. Set if the last arithmetic operation carried (addition) or borrowed (subtraction) a bit beyond the size of the register
    C = 0,

    /// Zero Flag. Set if the result of an operation is Zero (0).
    Z = 1,

    /// Negative Flag. Set if the result of an operation is negative
    N = 2,

    /// Overflow Flag. Set if signed arithmetic operations result in a value too large for the register to contain
    O = 3,

    /// Interruption Flag. Set if interrupts are enabled
    I = 4,
}

/// All integer register for the VM include general purpose and runtime specific register
#[derive(Debug, Copy, Clone)]
pub enum Register {
    /// 32-bit general purpose register
    A,
    B,
    C,
    D,
    E,
    F,

    /// 32-bit Stack pointer
    SP,

    /// 32-bit Base pointer (of subroutine call)
    BP,

    /// 32-bit Program Counter
    PC,

    /// Current Program State Register/Flags
    CPSR,

    /// NO-USE: Helper enum to get the number of Register
    CountMark,
}

/// Addressing is the way the instruction get the memory
#[derive(Debug)]
pub enum Addressing {
    /// Register to Register
    Direct(Register),

    /// Absolute pointer to memory
    Absolute(u32),

    /// Const Value
    Immediate(i32),

    /// The register store the pointer to memory
    Indirect(Register),
}

impl Addressing {
    pub fn load(&self, vm: &VM) -> u32 {
        match self {
            Addressing::Direct(reg) => vm.registers[*reg as usize],
            Addressing::Absolute(addr) => vm.heap[*addr as usize],
            Addressing::Immediate(v) => *v as u32,
            Addressing::Indirect(reg) => vm.heap[vm.registers[*reg as usize] as usize],
        }
    }
}

/// VM instruction set
/// A custom instruction set i come up with myself :^)
/// while referencing x86 and ARM instruction
#[derive(Debug)]
pub enum Instruction {
    /* ===== Memory access instructions ===== */
    /// Generate Program Counter relative address
    ADR(Register, i32),

    /// Load an value base on `Addressing` to `Register`
    ///
    /// Set Z, N Flags according to the result
    LOAD(Register, Addressing),

    /// Store an value base on `Addressing` to the memory location in `Register`
    STORE(Register, Addressing),

    /// Push an value base on `Addressing` to the stack
    PUSH(Addressing),

    /// Pop an value off the stack to `Register`
    ///
    /// Set Z, N Flags according to the result
    POP(Register),

    /* ===== General data processing instructions ===== */
    /// Add the value of `Addressing` to `Register` as integer, store the result in `Register`
    ///
    /// Set C, Z, N, O Flags according to the result
    ADD(Register, Addressing),

    /// Subtract the value of `Addressing` to `Register` as integer, store the result in `Register`
    ///
    /// Set C, Z, N, O Flags according to the result
    SUB(Register, Addressing),

    /// Multiply the value of `Addressing` to `Register` as integer, store the result in `Register`
    ///
    /// Set Z, N Flags according to the result
    MUL(Register, Addressing),

    /// Divide the value of `Addressing` to `Register` as integer, store the result in `Register`
    ///
    /// Set Z, N Flags according to the result
    DIV(Register, Addressing),

    /// Perform logical AND operator on `Addressing` and `Register` store the result in `Register`
    ///
    /// Set Z, N Flags according to the result
    AND(Register, Addressing),

    /// Perform logical OR operator on `Addressing` and `Register` store the result in `Register`
    ///
    /// Set Z, N Flags according to the result
    OR(Register, Addressing),

    /// Perform logical XOR operator on `Addressing` and `Register` store the result in `Register`
    ///
    /// Set Z, N Flags according to the result
    XOR(Register, Addressing),

    /// Flip all the bit in the register
    ///
    /// Set Z, N Flags according to the result
    NOT(Register),

    /// Performs the two's complement negation on `Register`
    NEG(Register),

    /// Shift the bit in the `Register` to the left, padding the resulting empty bit positions with zeros
    ///
    /// Set Z, N Flags according to the result
    LSL(Register, Addressing),

    /// Shift the bit in the `Register` to the right logically (ignore sign bit),
    /// padding the resulting empty bit positions with zeros
    ///
    /// Set Z, N Flags according to the result
    ///
    /// Set C Flag to the last bit shifted out
    LSR(Register, Addressing),

    /// Shift the bit in the `Register` to the right arithmetically (perserve sign bit),
    /// padding the resulting empty bit positions with zeros.
    ///
    /// Set Z, N Flags according to the result
    ///
    /// Set C Flag to the last bit shifted out
    ASR(Register, Addressing),

    /// Compare the value of `Addressing` to `Register` by doing subtraction
    ///
    /// Set Z, N Flags according to the result
    ///
    /// Equivalent to `Sub` instruction but the result is discarded
    CMP(Register, Addressing),

    /* ===== Branch and control instructions ===== */
    /// Jump to the address value of `Addressing`
    JMP(Addressing),

    /// Jump to the address value of `Addressing` if Z == 0
    JE(Addressing),

    /// Jump to the address value of `Addressing` if Z == 1
    JNE(Addressing),

    /// Jump to the address value of `Addressing` if Z == 0 && N == 0
    JG(Addressing),

    /// Jump to the address value of `Addressing` if Z == 1 && N == 0
    JGE(Addressing),

    /// Jump to the address value of `Addressing` if Z == 0 && N == 1
    JL(Addressing),

    /// Jump to the address value of `Addressing` if Z == 1 && N == 1
    JLE(Addressing),

    /// Push current Program Counter to the stack then jump to the address value of `Addressing`,
    CALL(Addressing),

    /// Pop the address from the stack then jump to that address
    RET,

    /* ===== Miscellaneous instructions ===== */
    /// Supervisor call, jump to native function (maybe a system call)
    SVC(i32),
}

pub struct VM<'a> {
    registers: [u32; Register::CountMark as usize],
    program: &'a [Instruction],
    stack: Vec<u32>,
    heap: Vec<u32>,
}

impl<'a> VM<'a> {
    pub fn new(program: &'a [Instruction]) -> VM<'a> {
        VM {
            registers: [0u32; Register::CountMark as usize],
            program,
            stack: Vec::new(),
            heap: vec![0; 4096],
        }
    }

    pub fn exec<F>(&mut self, mut svc: F)
    where
        F: FnMut(&mut Self, i32) + 'static,
    {
        self.registers[Register::PC as usize] = 0;

        loop {
            let instruction = &self.program[self.registers[Register::PC as usize] as usize];

            match instruction {
                Instruction::ADR(reg, offset) => {
                    self.registers[*reg as usize] =
                        (self.registers[Register::PC as usize] as i32 + *offset) as u32;
                }
                Instruction::LOAD(reg, addr) => {
                    let value = addr.load(self);
                    self.registers[*reg as usize] = value;
                    // TODO: Set N, Z flag
                }
                Instruction::STORE(reg, addr) => {
                    self.heap[self.registers[*reg as usize] as usize] = addr.load(self);
                }
                Instruction::PUSH(addr) => {
                    self.stack.push(addr.load(self));
                }
                Instruction::POP(reg) => {
                    let value = self.stack.pop().unwrap();
                    self.registers[*reg as usize] = value;
                    // TODO: Set N, Z flag
                }
                Instruction::ADD(_, _) => {}
                Instruction::SUB(_, _) => {}
                Instruction::MUL(_, _) => {}
                Instruction::DIV(_, _) => {}
                Instruction::AND(_, _) => {}
                Instruction::OR(_, _) => {}
                Instruction::XOR(_, _) => {}
                Instruction::NOT(_) => {}
                Instruction::NEG(_) => {}
                Instruction::LSL(_, _) => {}
                Instruction::LSR(_, _) => {}
                Instruction::ASR(_, _) => {}
                Instruction::CMP(_, _) => {}
                Instruction::JMP(_) => {}
                Instruction::JE(_) => {}
                Instruction::JNE(_) => {}
                Instruction::JG(_) => {}
                Instruction::JGE(_) => {}
                Instruction::JL(_) => {}
                Instruction::JLE(_) => {}
                Instruction::CALL(addr) => {
                    let ret_addr = self.registers[Register::PC as usize] + 1;
                    self.stack.push(ret_addr);
                    self.registers[Register::PC as usize] = addr.load(self);
                }
                Instruction::RET => {
                    self.registers[Register::PC as usize] = self.stack.pop().unwrap();
                    continue;
                }
                Instruction::SVC(v) => (svc)(self, *v),
            }

            self.registers[Register::PC as usize] += 1;
        }
    }
}
