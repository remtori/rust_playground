use super::*;

pub trait BitIndexing {
    fn bit_count(&self) -> usize;

    fn set_bit(&mut self, index: usize, value: u8);

    fn get_bit(&self, index: usize) -> Option<u8>;

    fn bit_at(&self, index: usize) -> u8 {
        self.get_bit(index).unwrap_or(0u8)
    }
}

pub struct BitIter<'a, T>
    where T: BitIndexing
{
    value: &'a T,
    index: usize,
}

impl<'a, T> BitIter<'a, T>
    where T: BitIndexing
{
    pub fn new(value: &'a T) -> BitIter<'a, T> {
        BitIter {
            value,
            index: 0,
        }
    }
}

impl<'a, T> Iterator for BitIter<'a, T>
    where T: BitIndexing
{
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let bit = self.value.get_bit(self.index);
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

        let bit = self.value.get_bit(len - self.index - 1);
        self.index += 1;
        bit
    }
}



impl BitIndexing for BigUInt {
    fn bit_count(&self) -> usize {
        self.bit_count
    }

    fn get_bit(&self, index: usize) -> Option<u8> {
        if index >= self.bit_count {
            return None;
        }

        let offset = index % Self::CHUNK_BIT_SIZE;
        let chunk_index = index / Self::CHUNK_BIT_SIZE;
        // println!("index={}, chunk_index={}, offset={}", index, chunk_index, offset);
        self.chunks.get(chunk_index).map(|v| ((*v >> offset) & 1) as u8)
    }

    fn set_bit(&mut self, index: usize, value: u8) {
        let offset = index % Self::CHUNK_BIT_SIZE;
        let chunk_index = index / Self::CHUNK_BIT_SIZE;
        let bit = 1 << offset;

        if chunk_index >= self.chunks.len() {
            self.chunks.extend(iter::repeat(0).take(chunk_index - self.chunks.len() + 1));
        }

        if let Some(chunk) = self.chunks.get_mut(chunk_index) {
            if value > 0 {
                *chunk |= bit;
                self.bit_count = cmp::max(self.bit_count, index + 1);
            } else {
                *chunk &= !bit;
            }
        } else {
            unreachable!();
        }
    }
}
