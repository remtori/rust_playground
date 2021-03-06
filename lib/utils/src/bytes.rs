pub struct BytesRef<'b> {
    buffer: &'b [u8],
    offset: usize,
}

impl<'b> BytesRef<'b> {
    pub fn peek(&self) -> Option<u8> {
        if self.offset < self.buffer.len() {
            Some(self.buffer[self.offset])
        } else {
            None
        }
    }

    pub fn peek_str(&self, size: usize) -> Option<&'b [u8]> {
        if self.offset + size <= self.buffer.len() {
            Some(&self.buffer[self.offset..(self.offset + size)])
        } else {
            None
        }
    }

    pub fn consume(&mut self, char: u8) -> Option<bool> {
        if let Some(c) = self.peek() {
            if c == char {
                self.offset += 1;
                return Some(true);
            }

            return Some(false);
        }

        None
    }

    pub fn consume_str(&mut self, str: &[u8]) -> Option<bool> {
        if let Some(s) = self.peek_str(str.len()) {
            if s == str {
                self.offset += str.len();
                return Some(true);
            }

            return Some(false);
        }

        None
    }

    pub fn skip(&mut self, size: usize) -> bool {
        if self.offset + size <= self.buffer.len() {
            self.offset += size;
            true
        } else {
            false
        }
    }

    pub fn consume_until(&mut self, c: u8) -> Option<&'b [u8]> {
        let start = self.offset;
        for idx in self.offset..self.buffer.len() {
            if c == self.buffer[idx] {
                self.offset = idx;
                return Some(&self.buffer[start..idx]);
            }
        }

        None
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl<'b> From<&'b [u8]> for BytesRef<'b> {
    fn from(buffer: &'b [u8]) -> Self {
        BytesRef { buffer, offset: 0 }
    }
}
