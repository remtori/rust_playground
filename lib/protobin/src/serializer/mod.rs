use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::{error::Result, WireType};

mod impls;

pub trait Serialize {
    fn wire_type(&self) -> WireType;

    fn serialize(&self, serializer: &mut Serializer) -> Result<()>;
}

pub fn to_bytes<T: Serialize>(value: &T) -> Result<Bytes> {
    let mut serializer = Serializer {
        buffer: BytesMut::new(),
    };

    value.serialize(&mut serializer)?;

    Ok(serializer.buffer.to_bytes())
}

pub struct Serializer {
    buffer: BytesMut,
}

impl Serializer {
    pub fn write_bool(&mut self, value: bool) -> Result<()> {
        self.buffer.put_u8(if value { 1 } else { 0 });
        Ok(())
    }

    pub fn write_u32(&mut self, value: u32) -> Result<()> {
        // Write as varint

        let mut value = value;
        while value > 0x7F {
            self.buffer.put_u8((value | 0x80) as u8);
            value >>= 7;
        }

        self.buffer.put_u8(value as u8);

        Ok(())
    }

    pub fn write_u64(&mut self, value: u64) -> Result<()> {
        // Write as varint

        let mut value = value;
        while value > 0x7F {
            self.buffer.put_u8((value | 0x80) as u8);
            value >>= 7;
        }

        self.buffer.put_u8(value as u8);

        Ok(())
    }

    pub fn write_i32(&mut self, v: i32) -> Result<()> {
        // Encode a 32-bit value with ZigZag encoding.
        self.write_u32(((v << 1) ^ (v >> 31)) as u32)
    }

    pub fn write_i64(&mut self, v: i64) -> Result<()> {
        // Encode a 64-bit value with ZigZag encoding.
        self.write_u64(((v << 1) ^ (v >> 63)) as u64)
    }

    pub fn write_f32(&mut self, v: f32) -> Result<()> {
        self.buffer.put_f32(v);
        Ok(())
    }

    pub fn write_f64(&mut self, v: f64) -> Result<()> {
        self.buffer.put_f64(v);
        Ok(())
    }

    pub fn write_char(&mut self, v: char) -> Result<()> {
        self.write_u32(v as u32)
    }

    pub fn write_str(&mut self, v: &str) -> Result<()> {
        self.write_bytes(v.as_bytes())
    }

    pub fn write_bytes(&mut self, v: &[u8]) -> Result<()> {
        debug_assert!(v.len() < u32::MAX as usize);

        // Len
        self.write_u32(v.len() as u32)?;

        // Bytes
        self.buffer.put_slice(v);

        Ok(())
    }

    pub fn write_field<T>(&mut self, field_id: u32, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.write_u32((field_id << 3) | (value.wire_type() as u32))?;
        value.serialize(self)?;
        Ok(())
    }
}
