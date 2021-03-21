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
#[derive(Debug)]
pub enum Register {
    /// 32-bit general purpose register
    A,
    B,
    C,
    D,
    E,
    F,

    /// 32-bit Stack pointer
    Sp,

    /// 32-bit Base pointer (of subroutine call)
    Bp,

    /// 32-bit Program Counter
    Pc,

    /// Current Program State Register/Flags
    Cpsr,
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

/// VM instruction set
/// A custom instruction set i come up with myself :^)
/// while referencing x86 and ARM instruction
#[derive(Debug)]
pub enum Instruction {
    /* ===== Memory access instructions ===== */

    /// Generate Program Counter relative address
    Adr(Register, i32),

    /// Load an value base on `Addressing` to `Register`
    /// Set Z, N Flags according to the result
    Load(Register, Addressing),

    /// Store an value base on `Addressing` to the memory location in `Register`
    Store(Register, Addressing),

    /// Push an value base on `Addressing` to the stack
    Push(Addressing),

    /// Pop an value off the stack to `Register`
    /// Set Z, N Flags according to the result
    Pop(Register),

    /* ===== General data processing instructions ===== */

    /// Add the value of `Addressing` to `Register` as integer, store the result in `Register`
    /// Set C, Z, N, O Flags according to the result
    Add(Register, Addressing),

    /// Subtract the value of `Addressing` to `Register` as integer, store the result in `Register`
    /// Set C, Z, N, O Flags according to the result
    Sub(Register, Addressing),

    /// Multiply the value of `Addressing` to `Register` as integer, store the result in `Register`
    /// Set Z, N Flags according to the result
    Mul(Register, Addressing),

    /// Divide the value of `Addressing` to `Register` as integer, store the result in `Register`
    /// Set Z, N Flags according to the result
    Div(Register, Addressing),

    /// Perform logical AND operator on `Addressing` and `Register` store the result in `Register`
    /// Set Z, N Flags according to the result
    And(Register, Addressing),

    /// Perform logical OR operator on `Addressing` and `Register` store the result in `Register`
    /// Set Z, N Flags according to the result
    Or(Register, Addressing),

    /// Perform logical XOR operator on `Addressing` and `Register` store the result in `Register`
    /// Set Z, N Flags according to the result
    Xor(Register, Addressing),

    /// Flip all the bit in the register
    /// Set Z, N Flags according to the result
    Not(Register),

    /// Performs the two's complement negation on `Register`
    Neg(Register),

    /// Shift the bit in the `Register` to the left, padding the resulting empty bit positions with zeros
    /// Set Z, N Flags according to the result
    Lsl(Register, Addressing),

    /// Shift the bit in the `Register` to the right logically (ignore sign bit),
    /// padding the resulting empty bit positions with zeros
    /// Set Z, N Flags according to the result
    /// Set C Flag to the last bit shifted out
    Lsr(Register, Addressing),

    /// Shift the bit in the `Register` to the right arithmetically (perserve sign bit),
    /// padding the resulting empty bit positions with zeros.
    /// Set Z, N Flags according to the result
    /// Set C Flag to the last bit shifted out
    Asr(Register, Addressing),

    /// Compare the value of `Addressing` to `Register` by doing subtraction
    /// Set Z, N Flags according to the result
    /// Equivalent to `Sub` instruction but the result is discarded
    Cmp(Register, Addressing),

    /* ===== Branch and control instructions ===== */

    /// Jump to the address value of `Addressing`
    Jmp(Addressing),

    /// Jump to the address value of `Addressing` if Z == 0
    Je(Addressing),

    /// Jump to the address value of `Addressing` if Z == 1
    Jne(Addressing),

    /// Jump to the address value of `Addressing` if Z == 0 && N == 0
    Jg(Addressing),

    /// Jump to the address value of `Addressing` if Z == 1 && N == 0
    Jge(Addressing),

    /// Jump to the address value of `Addressing` if Z == 0 && N == 1
    Jl(Addressing),

    /// Jump to the address value of `Addressing` if Z == 1 && N == 1
    Jle(Addressing),

    /// Push current Program Counter to the stack then jump to the address value of `Addressing`,
    Call(Addressing),

    /// Pop the address from the stack then jump to that address
    Ret,

    /* ===== Miscellaneous instructions ===== */

    /// Supervisor call, jump to native function (maybe a system call)
    Svc(i32)
}
