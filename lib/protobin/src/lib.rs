mod deserializer;
mod error;
mod serializer;

pub use deserializer::*;
pub use error::*;
pub use serializer::*;

pub use protobin_derive::Message;

pub enum WireType {
    VarInt = 0,
    VarUInt = 1,
    F32 = 2,
    F64 = 3,
    VarLen = 4,
}

impl From<u32> for WireType {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::VarInt,
            1 => Self::F32,
            2 => Self::F64,
            3 => Self::VarLen,
            _ => panic!("Unknown WireType"),
        }
    }
}
