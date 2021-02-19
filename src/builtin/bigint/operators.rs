use core::ops;
use super::*;

impl ops::BitAnd<BigUInt> for BigUInt {
    type Output = BigUInt;

    fn bitand(self, rhs: BigUInt) -> Self::Output {
        // `out` is the one with fewer bit
        let (mut out, rhs) = if self.bit_count() < rhs.bit_count() {
            (self, rhs)
        } else {
            (rhs, self)
        };

        for (idx, chunk) in out.chunks.iter_mut().enumerate() {
            *chunk &= rhs.chunk_at(idx);
        }

        out
    }
}

impl ops::BitAndAssign<BigUInt> for BigUInt {
    fn bitand_assign(&mut self, rhs: BigUInt) {
        for (idx, chunk) in self.chunks.iter_mut().enumerate() {
            *chunk &= rhs.chunk_at(idx);
        }
    }
}

impl ops::BitOr<BigUInt> for BigUInt {
    type Output = BigUInt;

    fn bitor(self, rhs: BigUInt) -> Self::Output {
        // `out` is the one with more bit
        let (mut out, rhs) = if self.bit_count() > rhs.bit_count() {
            (self, rhs)
        } else {
            (rhs, self)
        };

        for (idx, chunk) in out.chunks.iter_mut().enumerate() {
            *chunk |= rhs.chunk_at(idx);
        }

        out
    }
}

impl ops::BitOrAssign for BigUInt {
    fn bitor_assign(&mut self, rhs: Self) {
        self.pad_to(rhs.bit_count());
        for (idx, chunk) in self.chunks.iter_mut().enumerate() {
            *chunk |= rhs.chunk_at(idx);
        }
    }
}

impl ops::BitXor<BigUInt> for BigUInt {
    type Output = BigUInt;

    fn bitxor(self, rhs: BigUInt) -> Self::Output {
        // `out` is the one with more bit
        let (mut out, rhs) = if self.bit_count() > rhs.bit_count() {
            (self, rhs)
        } else {
            (rhs, self)
        };

        for (idx, chunk) in out.chunks.iter_mut().enumerate() {
            *chunk ^= rhs.chunk_at(idx);
        }

        out
    }
}

impl ops::BitXorAssign for BigUInt {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.pad_to(rhs.bit_count());
        for (idx, chunk) in self.chunks.iter_mut().enumerate() {
            *chunk ^= rhs.chunk_at(idx);
        }
    }
}

impl ops::Not for BigUInt {
    type Output = BigUInt;

    fn not(self) -> Self::Output {
        let mut out = self;

        for chunk in out.chunks.iter_mut() {
            *chunk = !(*chunk);
        }

        out
    }
}

impl ops::Shl<usize> for BigUInt {
    type Output = BigUInt;

    fn shl(self, amount: usize) -> Self::Output {
        let mut out = self;
        ops::ShlAssign::shl_assign(&mut out, amount);
        out
    }
}

impl ops::ShlAssign<usize> for BigUInt {
    fn shl_assign(&mut self, amount: usize) {
        self.pad_to(self.bit_count() + amount);

        let shift_chunk = amount / Self::CHUNK_BIT_SIZE;
        let shift_amount = amount % Self::CHUNK_BIT_SIZE;

        self.chunks.extend(iter::repeat(0).take(shift_chunk));

        let mut remainder = 0;
        for chunk in self.chunks.iter_mut() {
            let tmp = *chunk >> (Self::CHUNK_BIT_SIZE - shift_amount);
            *chunk <<= shift_amount;
            *chunk |= remainder;
            remainder = tmp;
        }

        self.chunks.rotate_left(shift_chunk);
        self.bit_count += amount;
    }
}

impl ops::Shr<usize> for BigUInt {
    type Output = BigUInt;

    fn shr(self, amount: usize) -> Self::Output {
        let mut out = self;
        ops::ShrAssign::shr_assign(&mut out, amount);
        out
    }
}

impl ops::ShrAssign<usize> for BigUInt {
    fn shr_assign(&mut self, amount: usize) {
        if amount >= self.bit_count() {
            self.clear();
            return;
        }

        let shift_chunk = amount / Self::CHUNK_BIT_SIZE;
        let shift_amount = amount % Self::CHUNK_BIT_SIZE;

        self.chunks.rotate_right(shift_chunk);
        self.chunks.truncate(self.chunks.len() - shift_chunk);

        let mut remainder = 0u32;
        for chunk in self.chunks.iter_mut().rev() {
            let tmp = *chunk << (Self::CHUNK_BIT_SIZE - shift_amount);
            *chunk >>= shift_amount;
            *chunk |= remainder;
            remainder = tmp;
        }

        self.bit_count -= amount;
    }
}

impl ops::Add<BigUInt> for BigUInt {
    type Output = BigUInt;

    fn add(self, rhs: BigUInt) -> Self::Output {
        let (mut out, rhs) = if self.bit_count() > rhs.bit_count() {
            (self, rhs)
        } else {
            (rhs, self)
        };

        ops::AddAssign::add_assign(&mut out, rhs);

        out
    }
}

impl ops::AddAssign<BigUInt> for BigUInt {
    fn add_assign(&mut self, rhs: BigUInt) {
        self.pad_to(rhs.bit_count());

        let mut remainder = 0;
        for (idx, chunk) in self.chunks.iter_mut().enumerate() {
            let sum = (*chunk as u64) + (rhs.chunk_at(idx) as u64) + remainder;
            remainder = sum >> Self::CHUNK_BIT_SIZE;
            *chunk = sum as u32;
        }

        if remainder > 0 {
            let remainder = remainder as u32;
            self.chunks.push(remainder);
        }

        self.adjust_bit_count();
    }
}

impl ops::Add<u32> for BigUInt {
    type Output = BigUInt;

    fn add(self, rhs: u32) -> Self::Output {
        let mut out = self;
        ops::AddAssign::add_assign(&mut out, rhs);
        out
    }
}

impl ops::AddAssign<u32> for BigUInt {
    fn add_assign(&mut self, rhs: u32) {
        let mut remainder = rhs as u64;
        for chunk in self.chunks.iter_mut() {
            let sum = (*chunk as u64) + remainder;
            remainder = sum >> Self::CHUNK_BIT_SIZE;
            *chunk = sum as u32;

            if remainder == 0 {
                break;
            }
        }

        if remainder > 0 {
            let remainder = remainder as u32;
            self.chunks.push(remainder);
        }

        self.adjust_bit_count();
    }
}

impl ops::Mul<u32> for BigUInt {
    type Output = BigUInt;

    fn mul(self, rhs: u32) -> Self::Output {
        let mut out = self;
        ops::MulAssign::mul_assign(&mut out, rhs);
        out
    }
}

impl ops::MulAssign<u32> for BigUInt {
    fn mul_assign(&mut self, rhs: u32) {
        let rhs = rhs as u64;
        let mut remainder = 0;
        for chunk in self.chunks.iter_mut() {
            let sum = (*chunk as u64) * rhs + remainder;
            remainder = sum >> Self::CHUNK_BIT_SIZE;
            *chunk = sum as u32;
        }

        if remainder > 0 {
            let remainder = remainder as u32;
            self.chunks.push(remainder);
        }

        self.adjust_bit_count();
    }
}
