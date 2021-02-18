use super::*;

#[test]
fn to_u64_clamp() {
    let n = BigUInt::from(12321);
    assert_eq!(n.to_u64_clamp(), 12321);

    let n = BigUInt::from(u64::MAX);
    assert_eq!(n.to_u64_clamp(), u64::MAX);
}

#[test]
fn to_string() {
    let n = BigUInt::from(12321);
    assert_eq!(n.to_string(), "12321");

    let n = BigUInt::from(1234567890123);
    assert_eq!(n.to_string(), "1234567890123");

    let n = BigUInt::from(i32::MAX as u64);
    assert_eq!(n.to_string(), format!("{}", i32::MAX));

    let n = BigUInt::from(u64::MAX);
    assert_eq!(n.to_string(), format!("{}", u64::MAX));
}

#[test]
fn to_bit_string() {
    let n = BigUInt::from(12321);
    assert_eq!(n.to_bit_string(), format!("{:b}", 12321));

    let n = BigUInt::from(1234567890123);
    assert_eq!(n.to_bit_string(), format!("{:b}", 1234567890123u64));

    let n = BigUInt::from(i32::MAX as u64);
    assert_eq!(n.to_bit_string(), format!("{:b}", i32::MAX));

    let n = BigUInt::from(u64::MAX);
    assert_eq!(n.to_bit_string(), format!("{:b}", u64::MAX));
}

#[test]
fn clear() {
    let mut n = BigUInt::from(12321);
    assert_eq!(n.to_u64_clamp(), 12321);

    n.clear();
    assert_eq!(n.to_u64_clamp(), 0);
}

#[test]
fn set_bit() {
    let mut n = BigUInt::from(12321);
    assert_eq!(n.to_u64_clamp(), 12321);

    n.set_bit(31, 0);
    assert_eq!(n.to_u64_clamp(), 12321u64);

    n.set_bit(32, 1);
    assert_eq!(n.to_u64_clamp(), 12321u64 | (1u64 << 32));
}

#[test]
fn ops_and() {
    let a = BigUInt::from(123321123321);
    let b = BigUInt::from(987789987789987);
    let out = a & b;
    assert_eq!(out.to_u64_clamp(), 123321123321 & 987789987789987);

    let a = BigUInt::from(1234567890123);
    let b = BigUInt::from(9876543210);
    let out = a & b;
    assert_eq!(out.to_u64_clamp(), 1234567890123 & 9876543210);

    let mut out = BigUInt::from(33333333333);
    out &= BigUInt::from(444444444);
    assert_eq!(out.to_u64_clamp(), 33333333333 & 444444444);
}

#[test]
fn ops_or() {
    let a = BigUInt::from(123321123321);
    let b = BigUInt::from(987789987789987);
    let out = a | b;
    assert_eq!(out.to_u64_clamp(), 123321123321 | 987789987789987);

    let a = BigUInt::from(1234567890123);
    let b = BigUInt::from(9876543210);
    let out = a | b;
    assert_eq!(out.to_u64_clamp(), 1234567890123 | 9876543210);

    let mut out = BigUInt::from(33333333333);
    out |= BigUInt::from(444444444);
    assert_eq!(out.to_u64_clamp(), 33333333333 | 444444444);
}

#[test]
fn ops_xor() {
    let a = BigUInt::from(123321123321);
    let b = BigUInt::from(987789987789987);
    let out = a ^ b;
    assert_eq!(out.to_u64_clamp(), 123321123321 ^ 987789987789987);

    let a = BigUInt::from(1234567890123);
    let b = BigUInt::from(9876543210);
    let out = a ^ b;
    assert_eq!(out.to_u64_clamp(), 1234567890123 ^ 9876543210);

    let mut out = BigUInt::from(33333333333);
    out ^= BigUInt::from(444444444);
    assert_eq!(out.to_u64_clamp(), 33333333333 ^ 444444444);
}

#[test]
fn ops_not() {
    let n = BigUInt::from(123321123321);
    assert_eq!((!n).to_u64_clamp(), !123321123321);

    let n = BigUInt::from(987789987789987);
    assert_eq!((!n).to_u64_clamp(), !987789987789987);

    let n = BigUInt::from(33333333333);
    assert_eq!((!n).to_u64_clamp(), !33333333333);
}
