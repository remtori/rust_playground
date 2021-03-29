use crate::{Error, Result, WireType};

mod impls;

pub trait Deserialize: Sized {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self>;

    fn deserialize_in_place(deserializer: &mut Deserializer, place: &mut Self) -> Result<()> {
        *place = Deserialize::deserialize(deserializer)?;
        Ok(())
    }
}

pub fn from_bytes<T: Deserialize>(bytes: &[u8]) -> Result<T> {
    let mut deserializer = Deserializer { buffer: bytes };
    Deserialize::deserialize(&mut deserializer)
}

pub struct Deserializer<'a> {
    buffer: &'a [u8],
}

impl<'a> Deserializer<'a> {
    pub fn new(buffer: &'a [u8]) -> Deserializer<'a> {
        Deserializer { buffer }
    }

    pub fn read_field(&mut self) -> Result<(u32, WireType)> {
        let encoded = self.read_u32()?;
        Ok((encoded >> 3, WireType::from(encoded & 7)))
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        Ok(self.get_and_advance()? != 0)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let temp = self.get_and_advance()?;
        let mut value = (temp & 0x7F) as u32;
        if temp < 128 {
            return Ok(value);
        }

        let temp = self.get_and_advance()?;
        value |= ((temp & 0x7F) as u32) << 7;
        if temp < 128 {
            return Ok(value);
        }

        let temp = self.get_and_advance()?;
        value |= ((temp & 0x7F) as u32) << 14;
        if temp < 128 {
            return Ok(value);
        }

        let temp = self.get_and_advance()?;
        value |= ((temp & 0x7F) as u32) << 21;
        if temp < 128 {
            return Ok(value);
        }

        let temp = self.get_and_advance()?;
        value |= ((temp & 0x7F) as u32) << 28;
        if temp < 128 {
            // We're reading the high bits of an unsigned varint. The byte we just read
            // also contains bits 33 through 35, which we're going to discard.
            return Ok(value);
        }

        // If we get here, we need to truncate coming bytes
        if self.get_and_advance()? >= 128
            && self.get_and_advance()? >= 128
            && self.get_and_advance()? >= 128
            && self.get_and_advance()? >= 128
            && self.get_and_advance()? >= 128
        {
            Err(Error::Message("Malform Varint".to_owned()))
        } else {
            Ok(value)
        }
    }

    pub fn read_u64(&mut self) -> Result<u64> {
        let mut shift = 0;
        let mut result = 0;

        while shift < 64 {
            let byte = self.get_and_advance()?;
            result |= ((byte & 0x7F) as u64) << shift;

            if byte < 128 {
                return Ok(result);
            }

            shift += 7;
        }

        Err(Error::Message("Malform Varint".to_owned()))
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        let v = self.read_u32()?;
        Ok((v >> 1) as i32 ^ -(v as i32 & 1))
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        let v = self.read_u64()?;
        Ok((v >> 1) as i64 ^ -(v as i64 & 1))
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        let mut buf = [0; 4];
        buf.copy_from_slice(self.slice_and_advance(4)?);

        Ok(f32::from_be_bytes(buf))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        let mut buf = [0; 8];
        buf.copy_from_slice(self.slice_and_advance(8)?);

        Ok(f64::from_be_bytes(buf))
    }

    pub fn read_char(&mut self) -> Result<char> {
        char::from_u32(self.read_u32()?).ok_or_else(|| Error::Message("invalid utf-8".to_owned()))
    }

    pub fn read_str(&mut self) -> Result<&'a str> {
        std::str::from_utf8(self.read_bytes()?)
            .map_err(|_| Error::Message("Invalid utf-8".to_owned()))
    }

    pub fn read_bytes(&mut self) -> Result<&'a [u8]> {
        let len = self.read_u32()? as usize;
        self.slice_and_advance(len)
    }

    #[inline]
    fn slice_and_advance(&mut self, n_byte: usize) -> Result<&'a [u8]> {
        if self.buffer.len() < n_byte {
            Err(Error::Message(format!(
                "EOF, need {} more bytes",
                n_byte - self.buffer.len()
            )))
        } else {
            let out = &self.buffer[..n_byte];
            self.buffer = &self.buffer[n_byte..];
            Ok(out)
        }
    }

    #[inline]
    fn get_and_advance(&mut self) -> Result<u8> {
        if self.buffer.is_empty() {
            Err(Error::Message("EOF, need 1 more bytes".to_owned()))
        } else {
            let out = self.buffer[0];
            self.buffer = &self.buffer[1..];
            Ok(out)
        }
    }
}
