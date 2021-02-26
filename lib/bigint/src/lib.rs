use std::*;
use std::mem::size_of;

pub mod indexing;
mod operators;
use indexing::*;

#[derive(Default, Clone)]
pub struct BigUInt {
    bit_count: usize,
    chunks: Vec<u32>,
}

impl BigUInt {
    const CHUNK_SIZE: usize = size_of::<u32>();
    const CHUNK_BIT_SIZE: usize = Self::CHUNK_SIZE * 8;

    pub fn new() -> Self {
        Self {
            bit_count: 0,
            chunks: Vec::new(),
        }
    }

    pub fn from_le_bytes(bytes: &[u8]) -> Self {
        let mut chunks: Vec<u32> = Vec::with_capacity((bytes.len() - 1) / Self::CHUNK_SIZE + 1);

        let mut bit_count = 0usize;
        let mut acc = 0u32;
        for (idx, byte) in bytes.iter().enumerate() {
            let off = idx % Self::CHUNK_SIZE;
            acc |= (*byte as u32) << (off * 8);
            if off == Self::CHUNK_SIZE - 1 {
                chunks.push(acc);
                acc = 0;
            }

            if *byte > 0 {
                bit_count = idx * 8 + (8 - byte.leading_zeros()) as usize;
            }
        }

        if acc > 0 {
            chunks.push(acc);
        }

        // println!("bit_count={}, chunks={:?}", bit_count, chunks);

        Self {
            bit_count,
            chunks
        }
    }

    pub fn from_str_radix(str: &str, radix: u32) -> Option<Self> {
        if !(2..36).contains(&radix) {
            return None;
        }

        let mut out = Self::new();
        for c in str.chars() {
            let digit = c.to_digit(radix)?;
            out = out * radix + digit;
        }

        Some(out)
    }

    pub fn to_u64_clamp(&self) -> u64 {
        let mut out = 0u64;

        // Iterate from MSB -> LSB
        for (idx, chunk) in self.chunks.iter().rev().enumerate() {
            out <<= Self::CHUNK_BIT_SIZE;
            out |= *chunk as u64;
            if (idx + 1) * Self::CHUNK_SIZE > size_of::<u64>() {
                return u64::MAX;
            }
        }

        out
    }

    pub fn is_zero(&self) -> bool {
        self.chunks.is_empty()
    }

    pub fn chunk_at(&self, chunk_index: usize) -> u32 {
        match self.chunks.get(chunk_index) {
            Some(v) => *v,
            None => 0,
        }
    }

    pub fn chunk_len(&self) -> usize {
        self.chunks.len()
    }

    pub fn bit_iter(&self) -> BitIter<Self> {
        BitIter::new(self)
    }

    pub fn pad_to(&mut self, bit_count: usize) {
        if bit_count >= self.bit_count {
            self.set_bit(bit_count - 1, 0);
        }
    }

    pub fn clear(&mut self) {
        self.bit_count = 0;
        self.chunks.clear();
    }

    fn adjust_bit_count(&mut self) {
        for (idx, chunk) in self.chunks.iter().rev().enumerate() {
            if *chunk > 0 {
                self.bit_count =
                    (self.chunks.len() - idx - 1) * Self::CHUNK_BIT_SIZE +
                    (Self::CHUNK_BIT_SIZE - chunk.leading_zeros() as usize);
                break;
            }
        }
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut out = String::with_capacity(self.bit_count() / 3);
        let mut divided = self.clone();
        let mut quotient = BigUInt::new();

        while !divided.is_zero() {
            quotient.clear();
            let mut remainder = 0u32;
            let len = divided.bit_count();

            // MSB -> LSB
            for (idx, bit) in divided.bit_iter().rev().enumerate() {
                remainder = remainder * 2 + (bit as u32);

                if remainder >= 10 {
                    quotient.set_bit(len - idx - 1, 1);
                    remainder -= 10;
                }
            }

            // println!("d={}\nq={}\nr={}", divided.to_bit_string(), quotient.to_bit_string(), remainder);

            out.push(char::from_digit(remainder %  10, 10).unwrap());
            std::mem::swap(&mut divided, &mut quotient);
        }

        out.chars().rev().collect()
    }

    pub fn to_bit_string(&self) -> String {
        let mut out = String::with_capacity(self.bit_count());

        // MSB -> LSB
        for bit in self.bit_iter().rev() {
            out.push( if bit > 0 { '1' } else { '0' } );
        }

        if out.is_empty() {
            out.push('0');
        }

        out
    }
}

impl From<u64> for BigUInt {
    fn from(value: u64) -> Self {
        Self::from_le_bytes(&value.to_le_bytes())
    }
}

impl fmt::Debug for BigUInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BigUInt({})", self.to_string())
    }
}

#[cfg(test)]
mod tests;
