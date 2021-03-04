#[derive(Debug, Eq, Ord, PartialOrd)]
pub struct CaseInsensitiveStrRef<'s>(&'s str);

impl<'s> std::cmp::PartialEq for CaseInsensitiveStrRef<'s> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl<'s> std::hash::Hash for CaseInsensitiveStrRef<'s> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for b in self.0.as_bytes() {
            state.write_u8(b.to_ascii_lowercase());
        }
    }
}

impl<'s> From<&'s str> for CaseInsensitiveStrRef<'s> {
    fn from(s: &'s str) -> Self {
        CaseInsensitiveStrRef(s)
    }
}

#[derive(Debug, Eq, Ord, PartialOrd)]
pub struct CaseInsensitiveString(String);

impl std::cmp::PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl std::hash::Hash for CaseInsensitiveString {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for b in self.0.as_bytes() {
            state.write_u8(b.to_ascii_lowercase());
        }
    }
}

impl From<String> for CaseInsensitiveString {
    fn from(s: String) -> Self {
        CaseInsensitiveString(s)
    }
}

impl From<&str> for CaseInsensitiveString {
    fn from(s: &str) -> Self {
        CaseInsensitiveString(s.to_string())
    }
}

#[derive(Debug, Eq, Ord, PartialOrd)]
pub struct CaseInsensitiveAsciiByte<'s>(&'s [u8]);

impl<'s> std::cmp::PartialEq for CaseInsensitiveAsciiByte<'s> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl<'s> std::hash::Hash for CaseInsensitiveAsciiByte<'s> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for b in self.0 {
            state.write_u8(b.to_ascii_lowercase())
        }
    }
}

impl<'s> From<&'s [u8]> for CaseInsensitiveAsciiByte<'s> {
    fn from(s: &'s [u8]) -> Self {
        CaseInsensitiveAsciiByte(s)
    }
}

impl<'s> From<&'s str> for CaseInsensitiveAsciiByte<'s> {
    fn from(s: &'s str) -> Self {
        CaseInsensitiveAsciiByte(s.as_bytes())
    }
}

// impl From<&'static [u8]> for CaseInsensitiveAsciiByte<'static> {
//     fn from(s: &'static [u8]) -> Self {
//         CaseInsensitiveAsciiByte(s)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_case_insensitive_string() {
        let mut map: HashMap<CaseInsensitiveString, u8> = HashMap::new();

        map.insert("a".into(), 1);
        assert_eq!(*map.get(&"a".into()).unwrap(), 1);

        map.insert("aaa".into(), 2);
        assert_eq!(*map.get(&"AAA".into()).unwrap(), 2);

        map.insert("aaB".into(), 3);
        assert_eq!(*map.get(&"AAb".into()).unwrap(), 3);
    }

    #[test]
    fn test_case_insensitive_string_ref() {
        let mut map: HashMap<CaseInsensitiveStrRef<'static>, u8> = HashMap::new();

        map.insert("a".into(), 1);
        assert_eq!(*map.get(&"a".into()).unwrap(), 1);

        map.insert("aaa".into(), 2);
        assert_eq!(*map.get(&"AAA".into()).unwrap(), 2);

        map.insert("aaB".into(), 3);
        assert_eq!(*map.get(&"AAb".into()).unwrap(), 3);
    }

    #[test]
    fn test_case_insensitive_ascii_byte() {
        let mut map: HashMap<CaseInsensitiveAsciiByte<'static>, u8> = HashMap::new();

        map.insert(b"a".as_ref().into(), 1);
        assert_eq!(*map.get(&b"a".as_ref().into()).unwrap(), 1);

        map.insert("aaa".into(), 2);
        assert_eq!(*map.get(&"AAA".into()).unwrap(), 2);

        map.insert("aaB".into(), 3);
        assert_eq!(*map.get(&"AAb".into()).unwrap(), 3);
    }
}
