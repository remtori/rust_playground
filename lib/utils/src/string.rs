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
}
