use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    fmt::Write,
    hash::{BuildHasher, Hash},
};

use crate::{Deserialize, Deserializer, Error, Result};

macro_rules! primitive_impl {
    ($ty:ident, $method:ident $($cast:tt)*) => {
        #[allow(clippy::needless_question_mark)]
        impl Deserialize for $ty {
            #[inline]
            fn deserialize(deserializer: &mut Deserializer) -> Result<Self>
            {
                Ok(deserializer.$method()? $($cast)*)
            }
        }
    }
}

primitive_impl!(bool, read_bool);
primitive_impl!(isize, read_i64 as isize);
primitive_impl!(i8, read_i32 as i8);
primitive_impl!(i16, read_i32 as i16);
primitive_impl!(i32, read_i32);
primitive_impl!(i64, read_i64);
primitive_impl!(usize, read_u64 as usize);
primitive_impl!(u8, read_u32 as u8);
primitive_impl!(u16, read_u32 as u16);
primitive_impl!(u32, read_u32);
primitive_impl!(u64, read_u64);
primitive_impl!(f32, read_f32);
primitive_impl!(f64, read_f64);
primitive_impl!(char, read_char);

impl Deserialize for &mut str {
    #[inline]
    fn deserialize(_deserializer: &mut Deserializer) -> Result<Self> {
        unimplemented!()
    }

    fn deserialize_in_place(deserializer: &mut Deserializer, place: &mut Self) -> Result<()> {
        if std::io::Write::write_all(
            &mut unsafe { place.as_bytes_mut() },
            deserializer.read_str()?.as_bytes(),
        )
        .is_err()
        {
            Err(Error::Message("Buffer too small".to_owned()))
        } else {
            Ok(())
        }
    }
}

impl Deserialize for String {
    #[inline]
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self> {
        Ok(deserializer.read_str()?.to_owned())
    }

    #[inline]
    fn deserialize_in_place(deserializer: &mut Deserializer, place: &mut Self) -> Result<()> {
        place.clear();
        if place.write_str(deserializer.read_str()?).is_err() {
            Err(Error::Message("Buffer too small".to_owned()))
        } else {
            Ok(())
        }
    }
}

impl<T: Copy + Default + Deserialize, const N: usize> Deserialize for [T; N] {
    #[inline]
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self> {
        let len = deserializer.read_u32()? as usize;
        debug_assert!(len == N);

        let mut out = [T::default(); N];
        for place in out.iter_mut() {
            T::deserialize_in_place(deserializer, place)?;
        }

        Ok(out)
    }
}

impl<'a, T: Deserialize> Deserialize for &'a mut [T] {
    #[inline]
    fn deserialize(_deserializer: &mut Deserializer) -> Result<Self> {
        unimplemented!()
    }

    #[inline]
    fn deserialize_in_place(deserializer: &mut Deserializer, place: &mut Self) -> Result<()> {
        let len = deserializer.read_u32()? as usize;
        debug_assert!(len < place.len());

        for i in 0..len {
            T::deserialize_in_place(deserializer, &mut place[i])?;
        }

        Ok(())
    }
}

macro_rules! seq_impl {
    ($ty:ident < T $(: $tbound1:ident $(+ $tbound2:ident)*)* $(, $typaram:ident : $bound:ident)*  >, $ctor:expr, $insert:ident) => {
        impl<T $(, $typaram)*> Deserialize for $ty<T $(, $typaram)*>
        where
            T: Deserialize $(+ $tbound1 $(+ $tbound2)*)*,
            $($typaram: $bound,)*
        {
            #[inline]
            fn deserialize(deserializer: &mut Deserializer) -> Result<Self> {
                let len = deserializer.read_u32()? as usize;
                let mut out = $ctor(len);

                for _ in 0..len {
                    out.$insert(T::deserialize(deserializer)?);
                }

                Ok(out)
            }

            #[inline]
            fn deserialize_in_place(deserializer: &mut Deserializer, place: &mut Self) -> Result<()> {
                place.clear();

                let len = deserializer.read_u32()? as usize;
                for _ in 0..len {
                    place.$insert(T::deserialize(deserializer)?);
                }

                Ok(())
            }
        }
    }
}

seq_impl!(BinaryHeap<T: Ord>, BinaryHeap::with_capacity, push);
seq_impl!(BTreeSet<T: Ord>, |_| BTreeSet::new(), insert);
seq_impl!(LinkedList<T>, |_| LinkedList::new(), push_back);
seq_impl!(Vec<T>, Vec::with_capacity, push);
seq_impl!(VecDeque<T>, VecDeque::with_capacity, push_back);

impl<T: Eq + Hash + Deserialize, H: BuildHasher + Default> Deserialize for HashSet<T, H> {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self> {
        let len = deserializer.read_u32()? as usize;
        let mut out = HashSet::with_capacity_and_hasher(len, H::default());

        for _ in 0..len {
            out.insert(T::deserialize(deserializer)?);
        }

        Ok(out)
    }

    fn deserialize_in_place(deserializer: &mut Deserializer, place: &mut Self) -> Result<()> {
        place.clear();

        let len = deserializer.read_u32()? as usize;
        for _ in 0..len {
            place.insert(T::deserialize(deserializer)?);
        }

        Ok(())
    }
}

impl<K: Ord + Deserialize, V: Deserialize> Deserialize for BTreeMap<K, V> {
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self> {
        let len = deserializer.read_u32()? as usize;
        let mut out = BTreeMap::new();

        for _ in 0..len {
            out.insert(K::deserialize(deserializer)?, V::deserialize(deserializer)?);
        }

        Ok(out)
    }

    fn deserialize_in_place(deserializer: &mut Deserializer, place: &mut Self) -> Result<()> {
        place.clear();

        let len = deserializer.read_u32()? as usize;
        for _ in 0..len {
            place.insert(K::deserialize(deserializer)?, V::deserialize(deserializer)?);
        }

        Ok(())
    }
}

impl<K: Eq + Hash + Deserialize, V: Deserialize, H: Default + BuildHasher> Deserialize
    for HashMap<K, V, H>
{
    fn deserialize(deserializer: &mut Deserializer) -> Result<Self> {
        let len = deserializer.read_u32()? as usize;
        let mut out = HashMap::with_capacity_and_hasher(len, H::default());

        for _ in 0..len {
            out.insert(K::deserialize(deserializer)?, V::deserialize(deserializer)?);
        }

        Ok(out)
    }

    fn deserialize_in_place(deserializer: &mut Deserializer, place: &mut Self) -> Result<()> {
        place.clear();

        let len = deserializer.read_u32()? as usize;
        for _ in 0..len {
            place.insert(K::deserialize(deserializer)?, V::deserialize(deserializer)?);
        }

        Ok(())
    }
}
