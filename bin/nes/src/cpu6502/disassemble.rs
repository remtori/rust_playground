use super::*;
use std::{collections::HashMap, ops::Range};

type Bus<'a> = &'a mut dyn crate::Bus;

impl Cpu6502 {
    pub fn disassemble(&mut self, bus: Bus, addr_range: Range<u16>) -> HashMap<u16, String> {
        let mut out = HashMap::new();
        let mut addr = addr_range.start;

        while addr <= addr_range.end {
            let line_addr = addr;

            let opcode = bus.read(addr, true);
            addr += 1;

            if addr >= addr_range.end {
                break;
            }

            let insn = lookup_instruction(opcode);

            let str = if is_same_addr_mode(insn.addr_mode, Cpu6502::imp) {
                format!("${:04x}: {} {{IMP}}", line_addr, insn.name)
            } else if is_same_addr_mode(insn.addr_mode, Cpu6502::imm) {
                let lo = bus.read(addr, true) as u16;
                addr += 1;

                format!("${:04x}: {} ${:02x} {{IMM}}", line_addr, insn.name, lo)
            } else if is_same_addr_mode(insn.addr_mode, Cpu6502::zp0) {
                let lo = bus.read(addr, true) as u16;
                addr += 1;

                format!("${:04x}: {} ${:02x} {{ZP0}}", line_addr, insn.name, lo)
            } else if is_same_addr_mode(insn.addr_mode, Cpu6502::zpx) {
                let lo = bus.read(addr, true) as u16;
                addr += 1;

                format!("${:04x}: {} ${:02x}, X {{ZPX}}", line_addr, insn.name, lo)
            } else if is_same_addr_mode(insn.addr_mode, Cpu6502::zpy) {
                let lo = bus.read(addr, true) as u16;
                addr += 1;

                format!("${:04x}: {} ${:02x}, Y {{ZPY}}", line_addr, insn.name, lo)
            } else if is_same_addr_mode(insn.addr_mode, Cpu6502::ind) {
                let lo = bus.read(addr, true) as u16;
                addr += 1;
                let hi = bus.read(addr, true) as u16;
                addr += 1;

                format!(
                    "${:04x}: {} (${:04x}) {{IND}}",
                    line_addr,
                    insn.name,
                    (hi << 8) | lo
                )
            } else if is_same_addr_mode(insn.addr_mode, Cpu6502::izx) {
                let lo = bus.read(addr, true) as u16;
                addr += 1;

                format!("${:04x}: {} (${:02x}), X {{IZX}}", line_addr, insn.name, lo)
            } else if is_same_addr_mode(insn.addr_mode, Cpu6502::izy) {
                let lo = bus.read(addr, true) as u16;
                addr += 1;

                format!("${:04x}: {} (${:2x}), Y {{IZY}}", line_addr, insn.name, lo)
            } else if is_same_addr_mode(insn.addr_mode, Cpu6502::abs) {
                let lo = bus.read(addr, true) as u16;
                addr += 1;
                let hi = bus.read(addr, true) as u16;
                addr += 1;

                format!(
                    "${:04x}: {} ${:04x} {{ABS}}",
                    line_addr,
                    insn.name,
                    (hi << 8) | lo
                )
            } else if is_same_addr_mode(insn.addr_mode, Cpu6502::abx) {
                let lo = bus.read(addr, true) as u16;
                addr += 1;
                let hi = bus.read(addr, true) as u16;
                addr += 1;

                format!(
                    "${:04x}: {} ${:04x} {{ABX}}",
                    line_addr,
                    insn.name,
                    (hi << 8) | lo
                )
            } else if is_same_addr_mode(insn.addr_mode, Cpu6502::aby) {
                let lo = bus.read(addr, true) as u16;
                addr += 1;
                let hi = bus.read(addr, true) as u16;
                addr += 1;

                format!(
                    "${:04x}: {} ${:04x} {{ABY}}",
                    line_addr,
                    insn.name,
                    (hi << 8) | lo
                )
            } else if is_same_addr_mode(insn.addr_mode, Cpu6502::rel) {
                let lo = bus.read(addr, true) as u16;
                addr += 1;

                format!(
                    "${:04x}: {} ${:02x} [${:04x}] {{REL}}",
                    line_addr,
                    insn.name,
                    lo,
                    addr + lo
                )
            } else {
                unreachable!()
            };

            out.insert(line_addr, str.to_uppercase());
        }

        out
    }
}
