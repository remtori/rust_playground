use super::*;
use utils::prelude::*;

impl Cpu6502 {
    /// utility function to fetch data
    fn fetch(&mut self, bus: Bus) -> u8 {
        if !is_same_addr_mode(lookup_instruction(self.opcode).addr_mode, Cpu6502::imp) {
            self.fetched = bus.read(self.addr_abs, false);
        }
        self.fetched
    }

    /// Invalid/Illegal instruction, behave identical to a NOP
    pub fn xxx(&mut self, _: Bus) -> u8 {
        warn!("Invalid instruction!");
        0
    }

    /// NOP
    pub fn nop(&mut self, _: Bus) -> u8 {
        0
    }

    /// Addition
    pub fn adc(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);

        // Do addition
        let temp = self.a as u16 + self.fetched as u16 + self.flag(Flags::C) as u16;

        // Set all the flags
        self.set_flag(Flags::C, temp > 255);
        self.set_zero_negative_flag((temp & 0x80) as u8);
        self.set_flag(
            Flags::V,
            (self.a as u16 ^ self.fetched as u16) & !(self.a as u16 ^ temp) & 0x0080 > 0,
        );

        // Save the result
        self.a = (temp & 0x00FF) as u8;
        1
    }

    /// Subtraction
    pub fn sbc(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);

        // Invert lower 8-bit
        let value = self.fetched as u16 ^ 0x00FF;

        // Do addition
        let temp = self.a as u16 + value + self.flag(Flags::C) as u16;

        // Set all the flags
        self.set_flag(Flags::C, temp > 255);
        self.set_zero_negative_flag((temp & 0x80) as u8);
        self.set_flag(
            Flags::V,
            (self.a as u16 ^ value) & !(self.a as u16 ^ temp) & 0x0080 > 0,
        );

        // Save the result
        self.a = (temp & 0x00FF) as u8;
        1
    }

    /// Branch if Carry Clear
    pub fn bcc(&mut self, _: Bus) -> u8 {
        if self.flag(Flags::C) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }

        0
    }

    /// Branch if Carry Set
    pub fn bcs(&mut self, _: Bus) -> u8 {
        if self.flag(Flags::C) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }

        0
    }

    /// Branch if Equal
    pub fn beq(&mut self, _: Bus) -> u8 {
        if self.flag(Flags::Z) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }

        0
    }

    /// Branch if Not Equal
    pub fn bne(&mut self, _: Bus) -> u8 {
        if self.flag(Flags::Z) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }

        0
    }

    /// Branch if Negative
    pub fn bmi(&mut self, _: Bus) -> u8 {
        if self.flag(Flags::N) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }

        0
    }

    /// branch if Positive
    pub fn bpl(&mut self, _: Bus) -> u8 {
        if self.flag(Flags::N) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }

        0
    }

    /// Branch if Overflow Clear
    pub fn bvc(&mut self, _: Bus) -> u8 {
        if self.flag(Flags::V) == 0 {
            self.cycles += 1;
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }

        0
    }

    /// Branch if Not Overflow Set
    pub fn bvs(&mut self, _: Bus) -> u8 {
        if self.flag(Flags::V) == 1 {
            self.cycles += 1;
            self.addr_abs = self.pc.wrapping_add(self.addr_rel);

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }

        0
    }

    /// Bit Test
    pub fn bit(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);
        let temp = self.fetched & self.a;

        self.set_zero_negative_flag(temp);
        self.set_flag(Flags::V, 0x40 & temp > 0);
        0
    }

    /// Force Interrupt
    pub fn brk(&mut self, bus: Bus) -> u8 {
        self.pc += 1;

        self.set_flag(Flags::I, true);
        // Store Program Counter
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

        // Store Status Register
        self.set_flag(Flags::B, true);
        bus.write(Self::BASE_STACK_PTR + self.stkp as u16, self.status);
        self.stkp -= 1;
        self.set_flag(Flags::B, false);

        // Jump to Interupt
        let lo = bus.read(Self::INTERUPT_PC + 0, false) as u16;
        let hi = bus.read(Self::INTERUPT_PC + 1, false) as u16;
        self.pc = (hi << 8) | lo;
        0
    }

    /// Arithmetic Shift Left
    pub fn asl(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);

        let temp = (self.fetched as u16) << 1;

        self.set_flag(Flags::C, (temp & 0xFF00) > 0);
        self.set_zero_negative_flag((temp & 0x00FF) as u8);

        let temp = (temp & 0x00FF) as u8;
        if is_same_addr_mode(lookup_instruction(self.opcode).addr_mode, Cpu6502::imp) {
            self.a = temp;
        } else {
            bus.write(self.addr_abs, temp);
        }

        0
    }

    /// Clear Carry Bit
    pub fn clc(&mut self, _: Bus) -> u8 {
        self.set_flag(Flags::C, false);
        0
    }

    /// Clear Decimal Bit
    pub fn cld(&mut self, _: Bus) -> u8 {
        self.set_flag(Flags::D, false);
        0
    }

    /// Clear Interupt Disable Bit
    pub fn cli(&mut self, _bus: Bus) -> u8 {
        self.set_flag(Flags::I, false);
        0
    }

    /// Clear Overflow Bit
    pub fn clv(&mut self, _bus: Bus) -> u8 {
        self.set_flag(Flags::V, false);
        0
    }

    /// Compare
    pub fn cmp(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);
        let temp = self.a as i8 - self.fetched as i8;
        self.set_flag(Flags::C, temp >= 0);
        self.set_zero_negative_flag(temp as u8);
        1
    }

    /// Compare X Register
    pub fn cpx(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);
        let temp = self.x as i8 - self.fetched as i8;
        self.set_flag(Flags::C, temp >= 0);
        self.set_zero_negative_flag(temp as u8);
        0
    }

    /// Compare Y Register
    pub fn cpy(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);
        let temp = self.y as i8 - self.fetched as i8;
        self.set_flag(Flags::C, temp >= 0);
        self.set_zero_negative_flag(temp as u8);
        0
    }

    /// Exclusive OR
    pub fn eor(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);
        self.a ^= self.fetched;
        self.set_zero_negative_flag(self.a);
        1
    }

    /// Decrement Memory - do decrement then set appropriate flag, no store
    pub fn dec(&mut self, bus: Bus) -> u8 {
        let temp = self.fetch(bus).wrapping_sub(1);
        bus.write(self.addr_abs, temp & 0x00FF);
        self.set_zero_negative_flag(temp);
        0
    }

    /// Decrement X Register
    pub fn dex(&mut self, _: Bus) -> u8 {
        self.x = self.x.wrapping_sub(1);
        self.set_zero_negative_flag(self.x);
        0
    }

    /// Decrement Y Register
    pub fn dey(&mut self, _: Bus) -> u8 {
        self.y = self.y.wrapping_sub(1);
        self.set_zero_negative_flag(self.y);
        0
    }

    /// Increment memory
    pub fn inc(&mut self, bus: Bus) -> u8 {
        let temp = self.fetch(bus).wrapping_add(1);
        bus.write(self.addr_abs, temp & 0x00FF);
        self.set_zero_negative_flag(temp);
        0
    }

    /// Increment X Register
    pub fn inx(&mut self, _bus: Bus) -> u8 {
        self.x = self.x.wrapping_add(1);
        self.set_zero_negative_flag(self.x);
        0
    }

    /// Increment Y Register
    pub fn iny(&mut self, _bus: Bus) -> u8 {
        self.y = self.y.wrapping_add(1);
        self.set_zero_negative_flag(self.y);
        0
    }

    /// Jump
    pub fn jmp(&mut self, _bus: Bus) -> u8 {
        self.pc = self.addr_abs;
        0
    }

    /// Jump to Subroutine
    pub fn jsr(&mut self, bus: Bus) -> u8 {
        self.pc -= 1;

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

        self.pc = self.addr_abs;
        0
    }

    /// Load Accumulator
    pub fn lda(&mut self, bus: Bus) -> u8 {
        self.a = self.fetch(bus);
        self.set_zero_negative_flag(self.a);
        1
    }

    /// Load X Register
    pub fn ldx(&mut self, bus: Bus) -> u8 {
        self.x = self.fetch(bus);
        self.set_zero_negative_flag(self.x);
        1
    }

    /// Load Y Register
    pub fn ldy(&mut self, bus: Bus) -> u8 {
        self.y = self.fetch(bus);
        self.set_zero_negative_flag(self.y);
        1
    }

    /// Logical Shift Right
    pub fn lsr(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);
        let temp = self.fetched >> 2;
        self.set_flag(Flags::C, self.fetched & 0x01 > 0);
        self.set_zero_negative_flag(temp);
        if is_same_addr_mode(lookup_instruction(self.opcode).addr_mode, Cpu6502::imp) {
            self.a = temp;
        } else {
            bus.write(self.addr_abs, temp);
        }

        0
    }

    /// Logical Inclusive OR
    pub fn ora(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);
        self.a |= self.fetched;
        self.set_zero_negative_flag(self.a);
        1
    }

    /// Logical AND
    pub fn and(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);
        self.a &= self.fetched;
        self.set_zero_negative_flag(self.a);
        1
    }

    /// Push A Register
    pub fn pha(&mut self, bus: Bus) -> u8 {
        // Hard-coded value for base stack pointer
        bus.write(Self::BASE_STACK_PTR + self.stkp as u16, self.a);
        self.stkp -= 1;
        0
    }

    /// Pop A Register
    pub fn pla(&mut self, bus: Bus) -> u8 {
        self.stkp += 1;
        self.a = bus.read(Self::BASE_STACK_PTR + self.stkp as u16, false);
        self.set_zero_negative_flag(self.a);
        0
    }

    /// Push Status Register
    pub fn php(&mut self, bus: Bus) -> u8 {
        bus.write(Self::BASE_STACK_PTR + self.stkp as u16, self.status);
        self.stkp -= 1;
        0
    }

    /// Pop Status Register
    pub fn plp(&mut self, bus: Bus) -> u8 {
        self.stkp += 1;
        self.status = bus.read(Self::BASE_STACK_PTR + self.stkp as u16, false);
        0
    }

    /// Rotate Left
    pub fn rol(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);

        let temp = (self.fetched << 1) as u16 | self.flag(Flags::C) as u16;
        self.set_flag(Flags::C, temp & 0xFF00 > 0);

        let temp = (temp & 0x00FF) as u8;
        self.set_zero_negative_flag(temp);

        if is_same_addr_mode(lookup_instruction(self.opcode).addr_mode, Cpu6502::imp) {
            self.a = temp;
        } else {
            bus.write(self.addr_abs, temp);
        }

        0
    }

    /// Rotate Right
    pub fn ror(&mut self, bus: Bus) -> u8 {
        self.fetch(bus);

        let temp = (self.flag(Flags::C) << 7) as u16 | (self.fetched >> 1) as u16;
        self.set_flag(Flags::C, temp & 0x01 > 0);

        let temp = (temp & 0x00FF) as u8;
        self.set_zero_negative_flag(temp);

        if is_same_addr_mode(lookup_instruction(self.opcode).addr_mode, Cpu6502::imp) {
            self.a = temp;
        } else {
            bus.write(self.addr_abs, temp);
        }

        0
    }

    /// Return from Interupt
    pub fn rti(&mut self, bus: Bus) -> u8 {
        self.stkp += 1;
        self.status = bus.read(Self::BASE_STACK_PTR + self.stkp as u16, false);
        self.set_flag(Flags::B, false);
        self.set_flag(Flags::U, false);

        self.stkp += 1;
        let lo = bus.read(Self::BASE_STACK_PTR + self.stkp as u16, false) as u16;
        self.stkp += 1;
        let hi = bus.read(Self::BASE_STACK_PTR + self.stkp as u16, false) as u16;

        self.pc = (hi << 8) | lo;
        0
    }

    /// Return from Subroutine
    pub fn rts(&mut self, bus: Bus) -> u8 {
        self.stkp += 1;
        let lo = bus.read(Self::BASE_STACK_PTR + self.stkp as u16, false) as u16;

        self.stkp += 1;
        let hi = bus.read(Self::BASE_STACK_PTR + self.stkp as u16, false) as u16;

        self.pc = ((hi << 8) | lo) + 1;

        0
    }

    /// Set Carry Flag
    pub fn sec(&mut self, _bus: Bus) -> u8 {
        self.set_flag(Flags::C, true);
        0
    }

    /// Set Decimal Flag
    pub fn sed(&mut self, _bus: Bus) -> u8 {
        self.set_flag(Flags::D, true);
        0
    }

    /// Set Interrupt Disable
    pub fn sei(&mut self, _bus: Bus) -> u8 {
        self.set_flag(Flags::I, true);
        0
    }

    /// Store A Register
    pub fn sta(&mut self, bus: Bus) -> u8 {
        bus.write(self.addr_abs, self.a);
        0
    }

    /// Store X Register
    pub fn stx(&mut self, bus: Bus) -> u8 {
        bus.write(self.addr_abs, self.x);
        0
    }

    /// Store Y Register
    pub fn sty(&mut self, bus: Bus) -> u8 {
        bus.write(self.addr_abs, self.y);
        0
    }

    /// Transfer Accumulator to X
    pub fn tax(&mut self, _bus: Bus) -> u8 {
        self.x = self.a;
        self.set_zero_negative_flag(self.x);
        0
    }

    /// Transfer Accumulator to Y
    pub fn tay(&mut self, _bus: Bus) -> u8 {
        self.y = self.a;
        self.set_zero_negative_flag(self.y);
        0
    }

    /// Transfer X to Accumulator
    pub fn txa(&mut self, _bus: Bus) -> u8 {
        self.a = self.x;
        self.set_zero_negative_flag(self.a);
        0
    }

    /// Transfer Y to Accumulator
    pub fn tya(&mut self, _bus: Bus) -> u8 {
        self.a = self.y;
        self.set_zero_negative_flag(self.a);
        0
    }

    /// Transfer Stack Pointer to X
    pub fn tsx(&mut self, _bus: Bus) -> u8 {
        self.x = self.stkp;
        self.set_zero_negative_flag(self.x);
        0
    }

    /// Transfer X to Stack Pointer
    pub fn txs(&mut self, _bus: Bus) -> u8 {
        self.stkp = self.x;
        0
    }
}
