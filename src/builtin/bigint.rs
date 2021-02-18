use std::*;
use std::mem::size_of;

#[derive(Debug, Default, Clone)]
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

    pub fn to_u64_clamp(&self) -> u64 {
        let mut out = 0u64;

        // Iterate from MSB -> LSB
        for (idx, chunk) in self.chunks.iter().rev().enumerate() {
            out <<= 32;
            out |= *chunk as u64;
            if idx > 1 {
                return u64::MAX;
            }
        }

        out
    }

    pub fn is_zero(&self) -> bool {
        self.chunks.is_empty()
    }

    pub fn bit_iter(&self) -> BitIter<Self> {
        BitIter {
            value: self,
            index: 0,
        }
    }

    pub fn clear(&mut self) {
        self.bit_count = 0;
        self.chunks.clear();
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    fn to_string(&self) -> String {
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

impl fmt::Display for BigUInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BigUInt({})", self.to_string())
    }
}

impl BitIndexing for BigUInt {
    fn bit_count(&self) -> usize {
        self.bit_count
    }

    fn bit_at(&self, index: usize) -> Option<u8> {
        if index >= self.bit_count {
            return None;
        }

        let offset = index % Self::CHUNK_BIT_SIZE;
        let chunk_index = index / Self::CHUNK_BIT_SIZE;
        // println!("index={}, chunk_index={}, offset={}", index, chunk_index, offset);
        self.chunks.get(chunk_index).map(|v| ((*v >> offset) & 1) as u8)
    }

    fn set_bit(&mut self, index: usize, value: u8) {
        if value > 0 {
            let offset = index % Self::CHUNK_BIT_SIZE;
            let chunk_index = index / Self::CHUNK_BIT_SIZE;
            let bit = 1 << offset;

            if chunk_index >= self.chunks.len() {
                self.chunks.extend(iter::repeat(0).take(chunk_index - self.chunks.len() + 1));
            }

            self.bit_count = cmp::max(self.bit_count, index + 1);
            if let Some(chunk) = self.chunks.get_mut(chunk_index) {
                *chunk |= bit;
            } else {
                unreachable!();
            }
        }
    }
}

pub trait BitIndexing {
    fn bit_count(&self) -> usize;
    fn bit_at(&self, index: usize) -> Option<u8>;
    fn set_bit(&mut self, index: usize, value: u8);
}

pub struct BitIter<'a, T>
    where T: BitIndexing
{
    value: &'a T,
    index: usize,
}

impl<'a, T> Iterator for BitIter<'a, T>
    where T: BitIndexing
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let bit = self.value.bit_at(self.index);
        self.index += 1;
        bit
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.value.bit_count()))
    }
}

impl<'a, T> DoubleEndedIterator for BitIter<'a, T>
    where T: BitIndexing
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let len = self.value.bit_count();
        if self.index >= len {
            return None;
        }

        let bit = self.value.bit_at(len - self.index - 1);
        self.index += 1;
        bit
    }
}
