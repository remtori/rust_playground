use super::*;

impl super::Cpu6502 {
    /// Relative addressing mode, only used by branch instruction
    /// i8 value relative to current program counter
    pub fn rel(&mut self, bus: Bus) -> u8 {
        self.addr_rel = bus.read(self.pc, false) as u16;
        self.pc += 1;

        if self.addr_rel & 0x80 > 0 {
            self.addr_rel |= 0xFF00;
        }

        0
    }

    /// Implied addressing mode, copy value from Accumulator Register to fetched
    pub fn imp(&mut self, _: Bus) -> u8 {
        self.fetched = self.a;
        0
    }

    /// Immediate addressing mode, the value is in the next byte
    pub fn imm(&mut self, _: Bus) -> u8 {
        self.addr_abs = self.pc;
        self.pc += 1;
        0
    }

    /// Zero Page addressing mode
    pub fn zp0(&mut self, bus: Bus) -> u8 {
        self.addr_abs = bus.read(self.pc, false) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    /// Zero Page with offset from register X
    pub fn zpx(&mut self, bus: Bus) -> u8 {
        self.addr_abs = bus.read(self.pc, false) as u16 + self.x as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    /// Zero Page with offset from register Y
    pub fn zpy(&mut self, bus: Bus) -> u8 {
        self.addr_abs = bus.read(self.pc, false) as u16 + self.y as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    /// Absolute addressing mode
    pub fn abs(&mut self, bus: Bus) -> u8 {
        let lo = bus.read(self.pc, false) as u16;
        self.pc += 1;
        let hi = bus.read(self.pc, false) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        0
    }

    /// Absolute with offset from register X
    pub fn abx(&mut self, bus: Bus) -> u8 {
        let lo = bus.read(self.pc, false) as u16;
        self.pc += 1;
        let hi = bus.read(self.pc, false) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.x as u16;

        // Page changed, need more cycles
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    /// Absolute with offset from register Y
    pub fn aby(&mut self, bus: Bus) -> u8 {
        let lo = bus.read(self.pc, false) as u16;
        self.pc += 1;
        let hi = bus.read(self.pc, false) as u16;
        self.pc += 1;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        // Page changed, need more cycles
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }

    /// Indirect addressing mode, read address from a pointer
    pub fn ind(&mut self, bus: Bus) -> u8 {
        let ptr_lo = bus.read(self.pc, false) as u16;
        self.pc += 1;
        let ptr_hi = bus.read(self.pc, false) as u16;
        self.pc += 1;

        let ptr = (ptr_hi << 8) | ptr_lo;

        let hi = if ptr_lo == 0x00FF {
            bus.read(ptr & 0xFF00, false) as u16
        } else {
            bus.read(ptr + 1, false) as u16
        };

        let lo = bus.read(ptr, false) as u16;
        self.addr_abs = (hi << 8) | lo;
        0
    }

    /// Indirect addressing mode with X offset
    pub fn izx(&mut self, bus: Bus) -> u8 {
        let t = bus.read(self.pc, false) as u16;
        self.pc += 1;

        let lo = bus.read((t + self.x as u16) & 0x00FF, false) as u16;
        let hi = bus.read((t + self.x as u16 + 1) & 0x00FF, false) as u16;

        self.addr_abs = (hi << 8) | lo;
        0
    }

    /// Indirect addressing mode with Y offset
    pub fn izy(&mut self, bus: Bus) -> u8 {
        let t = bus.read(self.pc, false) as u16;
        self.pc += 1;

        let lo = bus.read(t & 0x00FF, false) as u16;
        let hi = bus.read((t + 1) & 0x00FF, false) as u16;

        self.addr_abs = (hi << 8) | lo;
        self.addr_abs += self.y as u16;

        // Page changed, need more cycles
        if (self.addr_abs & 0xFF00) != (hi << 8) {
            1
        } else {
            0
        }
    }
}
