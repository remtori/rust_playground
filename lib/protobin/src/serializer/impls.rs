use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::{BuildHasher, Hash},
};

use crate::{Result, Serialize, Serializer, WireType};

macro_rules! primitive_impl {
    ($ty:ident, $wty:ident, $method:ident $($cast:tt)*) => {
        impl Serialize for $ty {
            #[inline]
            fn wire_type(&self) -> WireType {
                WireType::$wty
            }

            #[inline]
            fn serialize(&self, serializer: &mut Serializer) -> Result<()>
            {
                serializer.$method(*self $($cast)*)
            }
        }
    }
}

primitive_impl!(bool, VarInt, write_bool);
primitive_impl!(isize, VarInt, write_i64 as i64);
primitive_impl!(i8, VarInt, write_i32 as i32);
primitive_impl!(i16, VarInt, write_i32 as i32);
primitive_impl!(i32, VarInt, write_i32);
primitive_impl!(i64, VarInt, write_i64);
primitive_impl!(usize, VarUInt, write_u64 as u64);
primitive_impl!(u8, VarUInt, write_u32 as u32);
primitive_impl!(u16, VarUInt, write_u32 as u32);
primitive_impl!(u32, VarUInt, write_u32);
primitive_impl!(u64, VarUInt, write_u64);
primitive_impl!(f32, F32, write_f32);
primitive_impl!(f64, F64, write_f64);
primitive_impl!(char, VarUInt, write_char);

impl Serialize for str {
    #[inline]
    fn wire_type(&self) -> WireType {
        WireType::VarLen
    }

    #[inline]
    fn serialize(&self, serializer: &mut Serializer) -> Result<()> {
        serializer.write_bytes(self.as_bytes())
    }
}

impl Serialize for String {
    #[inline]
    fn wire_type(&self) -> WireType {
        WireType::VarLen
    }

    #[inline]
    fn serialize(&self, serializer: &mut Serializer) -> Result<()> {
        serializer.write_bytes(self.as_bytes())
    }
}

impl<T: Serialize, const N: usize> Serialize for [T; N] {
    #[inline]
    fn wire_type(&self) -> WireType {
        WireType::VarLen
    }

    #[inline]
    fn serialize(&self, serializer: &mut Serializer) -> Result<()> {
        debug_assert!(N < u32::MAX as usize);

        serializer.write_u32(N as u32)?;
        for e in self {
            e.serialize(serializer)?;
        }

        Ok(())
    }
}

impl<T: Serialize> Serialize for [T] {
    #[inline]
    fn wire_type(&self) -> WireType {
        WireType::VarLen
    }

    #[inline]
    fn serialize(&self, serializer: &mut Serializer) -> Result<()> {
        debug_assert!(self.len() < u32::MAX as usize);

        serializer.write_u32(self.len() as u32)?;
        for e in self {
            e.serialize(serializer)?;
        }

        Ok(())
    }
}

macro_rules! seq_impl {
    ($ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* $(, $typaram:ident : $bound:ident)* >) => {
        impl<T $(, $typaram)*> Serialize for $ty<T $(, $typaram)*>
        where
            T: Serialize $(+ $tbound1 $(+ $tbound2)*)*,
            $($typaram: $bound,)*
        {
            #[inline]
            fn wire_type(&self) -> WireType {
                WireType::VarLen
            }

            #[inline]
            fn serialize(&self, serializer: &mut Serializer) -> Result<()>
            {
                debug_assert!(self.len() < u32::MAX as usize);

                serializer.write_u32(self.len() as u32)?;
                for e in self {
                    e.serialize(serializer)?;
                }

                Ok(())
            }
        }
    }
}

seq_impl!(BinaryHeap<T: Ord>);
seq_impl!(BTreeSet<T: Ord>);
seq_impl!(HashSet<T: Eq + Hash, H: BuildHasher>);
seq_impl!(LinkedList<T>);
seq_impl!(Vec<T>);
seq_impl!(VecDeque<T>);

macro_rules! map_impl {
    ($ty:ident < K $(: $kbound1:ident $(+ $kbound2:ident)*)*, V $(, $typaram:ident : $bound:ident)* >) => {
        impl<K, V $(, $typaram)*> Serialize for $ty<K, V $(, $typaram)*>
        where
            K: Serialize $(+ $kbound1 $(+ $kbound2)*)*,
            V: Serialize,
            $($typaram: $bound,)*
        {
            #[inline]
            fn wire_type(&self) -> WireType {
                WireType::VarLen
            }

            #[inline]
            fn serialize(&self, serializer: &mut Serializer) -> Result<()>
            {
                debug_assert!(self.len() < u32::MAX as usize);

                serializer.write_u32(self.len() as u32)?;
                for (k, e) in self {
                    k.serialize(serializer)?;
                    e.serialize(serializer)?;
                }

                Ok(())
            }
        }
    }
}

map_impl!(BTreeMap<K: Ord, V>);
map_impl!(HashMap<K: Eq + Hash, V, H: BuildHasher>);
