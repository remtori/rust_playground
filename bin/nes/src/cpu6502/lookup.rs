use crate::cpu6502::Cpu6502;
use lazy_static::lazy_static;

pub fn lookup_instruction(opcode: u8) -> &'static Instruction {
    &INSN_LOOKUP[opcode as usize]
}

pub type InsnFunc = fn(&mut Cpu6502, bus: &mut crate::emulator::SystemBus) -> u8;

pub struct Instruction {
    pub name: &'static str,
    pub operate: InsnFunc,
    pub addr_mode: InsnFunc,
    pub cycles: u8,
}

impl Instruction {
    pub fn new(
        name: &'static str,
        operate: InsnFunc,
        addr_mode: InsnFunc,
        cycles: u8,
    ) -> Instruction {
        Instruction {
            name,
            operate,
            addr_mode,
            cycles,
        }
    }
}

lazy_static! {
    static ref INSN_LOOKUP: [Instruction; 256] = {
        type I = Instruction;
        [
            I::new("BRK", Cpu6502::brk, Cpu6502::imm, 7),
            I::new("ORA", Cpu6502::ora, Cpu6502::izx, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 3),
            I::new("ORA", Cpu6502::ora, Cpu6502::zp0, 3),
            I::new("ASL", Cpu6502::asl, Cpu6502::zp0, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 5),
            I::new("PHP", Cpu6502::php, Cpu6502::imp, 3),
            I::new("ORA", Cpu6502::ora, Cpu6502::imm, 2),
            I::new("ASL", Cpu6502::asl, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("ORA", Cpu6502::ora, Cpu6502::abs, 4),
            I::new("ASL", Cpu6502::asl, Cpu6502::abs, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("BPL", Cpu6502::bpl, Cpu6502::rel, 2),
            I::new("ORA", Cpu6502::ora, Cpu6502::izy, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("ORA", Cpu6502::ora, Cpu6502::zpx, 4),
            I::new("ASL", Cpu6502::asl, Cpu6502::zpx, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("CLC", Cpu6502::clc, Cpu6502::imp, 2),
            I::new("ORA", Cpu6502::ora, Cpu6502::aby, 4),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("ORA", Cpu6502::ora, Cpu6502::abx, 4),
            I::new("ASL", Cpu6502::asl, Cpu6502::abx, 7),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
            I::new("JSR", Cpu6502::jsr, Cpu6502::abs, 6),
            I::new("AND", Cpu6502::and, Cpu6502::izx, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("BIT", Cpu6502::bit, Cpu6502::zp0, 3),
            I::new("AND", Cpu6502::and, Cpu6502::zp0, 3),
            I::new("ROL", Cpu6502::rol, Cpu6502::zp0, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 5),
            I::new("PLP", Cpu6502::plp, Cpu6502::imp, 4),
            I::new("AND", Cpu6502::and, Cpu6502::imm, 2),
            I::new("ROL", Cpu6502::rol, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("BIT", Cpu6502::bit, Cpu6502::abs, 4),
            I::new("AND", Cpu6502::and, Cpu6502::abs, 4),
            I::new("ROL", Cpu6502::rol, Cpu6502::abs, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("BMI", Cpu6502::bmi, Cpu6502::rel, 2),
            I::new("AND", Cpu6502::and, Cpu6502::izy, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("AND", Cpu6502::and, Cpu6502::zpx, 4),
            I::new("ROL", Cpu6502::rol, Cpu6502::zpx, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("SEC", Cpu6502::sec, Cpu6502::imp, 2),
            I::new("AND", Cpu6502::and, Cpu6502::aby, 4),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("AND", Cpu6502::and, Cpu6502::abx, 4),
            I::new("ROL", Cpu6502::rol, Cpu6502::abx, 7),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
            I::new("RTI", Cpu6502::rti, Cpu6502::imp, 6),
            I::new("EOR", Cpu6502::eor, Cpu6502::izx, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 3),
            I::new("EOR", Cpu6502::eor, Cpu6502::zp0, 3),
            I::new("LSR", Cpu6502::lsr, Cpu6502::zp0, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 5),
            I::new("PHA", Cpu6502::pha, Cpu6502::imp, 3),
            I::new("EOR", Cpu6502::eor, Cpu6502::imm, 2),
            I::new("LSR", Cpu6502::lsr, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("JMP", Cpu6502::jmp, Cpu6502::abs, 3),
            I::new("EOR", Cpu6502::eor, Cpu6502::abs, 4),
            I::new("LSR", Cpu6502::lsr, Cpu6502::abs, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("BVC", Cpu6502::bvc, Cpu6502::rel, 2),
            I::new("EOR", Cpu6502::eor, Cpu6502::izy, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("EOR", Cpu6502::eor, Cpu6502::zpx, 4),
            I::new("LSR", Cpu6502::lsr, Cpu6502::zpx, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("CLI", Cpu6502::cli, Cpu6502::imp, 2),
            I::new("EOR", Cpu6502::eor, Cpu6502::aby, 4),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("EOR", Cpu6502::eor, Cpu6502::abx, 4),
            I::new("LSR", Cpu6502::lsr, Cpu6502::abx, 7),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
            I::new("RTS", Cpu6502::rts, Cpu6502::imp, 6),
            I::new("ADC", Cpu6502::adc, Cpu6502::izx, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 3),
            I::new("ADC", Cpu6502::adc, Cpu6502::zp0, 3),
            I::new("ROR", Cpu6502::ror, Cpu6502::zp0, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 5),
            I::new("PLA", Cpu6502::pla, Cpu6502::imp, 4),
            I::new("ADC", Cpu6502::adc, Cpu6502::imm, 2),
            I::new("ROR", Cpu6502::ror, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("JMP", Cpu6502::jmp, Cpu6502::ind, 5),
            I::new("ADC", Cpu6502::adc, Cpu6502::abs, 4),
            I::new("ROR", Cpu6502::ror, Cpu6502::abs, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("BVS", Cpu6502::bvs, Cpu6502::rel, 2),
            I::new("ADC", Cpu6502::adc, Cpu6502::izy, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("ADC", Cpu6502::adc, Cpu6502::zpx, 4),
            I::new("ROR", Cpu6502::ror, Cpu6502::zpx, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("SEI", Cpu6502::sei, Cpu6502::imp, 2),
            I::new("ADC", Cpu6502::adc, Cpu6502::aby, 4),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("ADC", Cpu6502::adc, Cpu6502::abx, 4),
            I::new("ROR", Cpu6502::ror, Cpu6502::abx, 7),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("STA", Cpu6502::sta, Cpu6502::izx, 6),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("STY", Cpu6502::sty, Cpu6502::zp0, 3),
            I::new("STA", Cpu6502::sta, Cpu6502::zp0, 3),
            I::new("STX", Cpu6502::stx, Cpu6502::zp0, 3),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 3),
            I::new("DEY", Cpu6502::dey, Cpu6502::imp, 2),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("TXA", Cpu6502::txa, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("STY", Cpu6502::sty, Cpu6502::abs, 4),
            I::new("STA", Cpu6502::sta, Cpu6502::abs, 4),
            I::new("STX", Cpu6502::stx, Cpu6502::abs, 4),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 4),
            I::new("BCC", Cpu6502::bcc, Cpu6502::rel, 2),
            I::new("STA", Cpu6502::sta, Cpu6502::izy, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("STY", Cpu6502::sty, Cpu6502::zpx, 4),
            I::new("STA", Cpu6502::sta, Cpu6502::zpx, 4),
            I::new("STX", Cpu6502::stx, Cpu6502::zpy, 4),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 4),
            I::new("TYA", Cpu6502::tya, Cpu6502::imp, 2),
            I::new("STA", Cpu6502::sta, Cpu6502::aby, 5),
            I::new("TXS", Cpu6502::txs, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 5),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 5),
            I::new("STA", Cpu6502::sta, Cpu6502::abx, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 5),
            I::new("LDY", Cpu6502::ldy, Cpu6502::imm, 2),
            I::new("LDA", Cpu6502::lda, Cpu6502::izx, 6),
            I::new("LDX", Cpu6502::ldx, Cpu6502::imm, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("LDY", Cpu6502::ldy, Cpu6502::zp0, 3),
            I::new("LDA", Cpu6502::lda, Cpu6502::zp0, 3),
            I::new("LDX", Cpu6502::ldx, Cpu6502::zp0, 3),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 3),
            I::new("TAY", Cpu6502::tay, Cpu6502::imp, 2),
            I::new("LDA", Cpu6502::lda, Cpu6502::imm, 2),
            I::new("TAX", Cpu6502::tax, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("LDY", Cpu6502::ldy, Cpu6502::abs, 4),
            I::new("LDA", Cpu6502::lda, Cpu6502::abs, 4),
            I::new("LDX", Cpu6502::ldx, Cpu6502::abs, 4),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 4),
            I::new("BCS", Cpu6502::bcs, Cpu6502::rel, 2),
            I::new("LDA", Cpu6502::lda, Cpu6502::izy, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 5),
            I::new("LDY", Cpu6502::ldy, Cpu6502::zpx, 4),
            I::new("LDA", Cpu6502::lda, Cpu6502::zpx, 4),
            I::new("LDX", Cpu6502::ldx, Cpu6502::zpy, 4),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 4),
            I::new("CLV", Cpu6502::clv, Cpu6502::imp, 2),
            I::new("LDA", Cpu6502::lda, Cpu6502::aby, 4),
            I::new("TSX", Cpu6502::tsx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 4),
            I::new("LDY", Cpu6502::ldy, Cpu6502::abx, 4),
            I::new("LDA", Cpu6502::lda, Cpu6502::abx, 4),
            I::new("LDX", Cpu6502::ldx, Cpu6502::aby, 4),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 4),
            I::new("CPY", Cpu6502::cpy, Cpu6502::imm, 2),
            I::new("CMP", Cpu6502::cmp, Cpu6502::izx, 6),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("CPY", Cpu6502::cpy, Cpu6502::zp0, 3),
            I::new("CMP", Cpu6502::cmp, Cpu6502::zp0, 3),
            I::new("DEC", Cpu6502::dec, Cpu6502::zp0, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 5),
            I::new("INY", Cpu6502::iny, Cpu6502::imp, 2),
            I::new("CMP", Cpu6502::cmp, Cpu6502::imm, 2),
            I::new("DEX", Cpu6502::dex, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("CPY", Cpu6502::cpy, Cpu6502::abs, 4),
            I::new("CMP", Cpu6502::cmp, Cpu6502::abs, 4),
            I::new("DEC", Cpu6502::dec, Cpu6502::abs, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("BNE", Cpu6502::bne, Cpu6502::rel, 2),
            I::new("CMP", Cpu6502::cmp, Cpu6502::izy, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("CMP", Cpu6502::cmp, Cpu6502::zpx, 4),
            I::new("DEC", Cpu6502::dec, Cpu6502::zpx, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("CLD", Cpu6502::cld, Cpu6502::imp, 2),
            I::new("CMP", Cpu6502::cmp, Cpu6502::aby, 4),
            I::new("NOP", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("CMP", Cpu6502::cmp, Cpu6502::abx, 4),
            I::new("DEC", Cpu6502::dec, Cpu6502::abx, 7),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
            I::new("CPX", Cpu6502::cpx, Cpu6502::imm, 2),
            I::new("SBC", Cpu6502::sbc, Cpu6502::izx, 6),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("CPX", Cpu6502::cpx, Cpu6502::zp0, 3),
            I::new("SBC", Cpu6502::sbc, Cpu6502::zp0, 3),
            I::new("INC", Cpu6502::inc, Cpu6502::zp0, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 5),
            I::new("INX", Cpu6502::inx, Cpu6502::imp, 2),
            I::new("SBC", Cpu6502::sbc, Cpu6502::imm, 2),
            I::new("NOP", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("???", Cpu6502::sbc, Cpu6502::imp, 2),
            I::new("CPX", Cpu6502::cpx, Cpu6502::abs, 4),
            I::new("SBC", Cpu6502::sbc, Cpu6502::abs, 4),
            I::new("INC", Cpu6502::inc, Cpu6502::abs, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("BEQ", Cpu6502::beq, Cpu6502::rel, 2),
            I::new("SBC", Cpu6502::sbc, Cpu6502::izy, 5),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 8),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("SBC", Cpu6502::sbc, Cpu6502::zpx, 4),
            I::new("INC", Cpu6502::inc, Cpu6502::zpx, 6),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 6),
            I::new("SED", Cpu6502::sed, Cpu6502::imp, 2),
            I::new("SBC", Cpu6502::sbc, Cpu6502::aby, 4),
            I::new("NOP", Cpu6502::nop, Cpu6502::imp, 2),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
            I::new("???", Cpu6502::nop, Cpu6502::imp, 4),
            I::new("SBC", Cpu6502::sbc, Cpu6502::abx, 4),
            I::new("INC", Cpu6502::inc, Cpu6502::abx, 7),
            I::new("???", Cpu6502::xxx, Cpu6502::imp, 7),
        ]
    };
}
