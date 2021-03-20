use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::{BuildHasher, Hash, Hasher},
    ops::Deref,
    sync::{Arc, Mutex, Weak},
};

use lazy_static::lazy_static;

lazy_static! {
    static ref STRING_CACHE: Mutex<HashMap<u64, Weak<Box<str>>>> = Mutex::new(HashMap::new());
}

#[derive(Clone)]
pub struct FlyString {
    hash: u64,
    value: Arc<Box<str>>,
}

impl Drop for FlyString {
    fn drop(&mut self) {
        if let Ok(mut cache) = STRING_CACHE.lock() {
            if let Some(weak) = cache.get(&self.hash) {
                // If we hold the last strong ref, remove its from cache
                if weak.strong_count() <= 1 {
                    cache.remove(&self.hash);
                }
            }
        }
    }
}

impl PartialEq for FlyString {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl Eq for FlyString {}

impl PartialOrd for FlyString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hash.partial_cmp(&other.hash)
    }
}

impl Ord for FlyString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hash.cmp(&other.hash)
    }
}

impl AsRef<str> for FlyString {
    fn as_ref(&self) -> &str {
        self.value.as_ref()
    }
}

impl Deref for FlyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.value.as_ref()
    }
}

impl Display for FlyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.value.as_ref())
    }
}

impl Debug for FlyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlyString{{{}: {}}}", self.hash, self.value)
    }
}

impl<'a, S: Into<&'a str>> From<S> for FlyString {
    fn from(s: S) -> Self {
        let mut cache = STRING_CACHE
            .lock()
            .expect("FlyString: STRING_CACHE lock failed");

        let str: &str = s.into();
        let hash = {
            let state = cache.hasher();
            let mut hasher = state.build_hasher();
            str.hash(&mut hasher);
            hasher.finish()
        };

        if let Some(weak_str) = cache.get(&hash) {
            if let Some(arc_str) = weak_str.upgrade() {
                return Self {
                    hash,
                    value: arc_str,
                };
            }
        }

        let arc_str = Arc::new(str.to_owned().into_boxed_str());
        cache.insert(hash, Arc::downgrade(&arc_str));

        Self {
            hash,
            value: arc_str,
        }
    }
}

#[cfg(test)]
mod test {
    use super::FlyString;

    #[test]
    fn to_and_from() {
        let fly_str = FlyString::from("abc");
        assert_eq!(&fly_str[..], "abc");

        let s = fly_str.to_string();
        assert_eq!(s, "abc");
    }
}
