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
