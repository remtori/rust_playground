// FIXME: These test are not really good and not cover all the edge cases
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

#[test]
fn ops_shift_left() {
    let n = BigUInt::from(1233211) << 5;
    assert_eq!(n.to_u64_clamp(), 1233211 << 5);

    let n = BigUInt::from(1233211) << 1;
    assert_eq!(n.to_u64_clamp(), 1233211 << 1);

    let n = BigUInt::from(2) << 14;
    assert_eq!(n.to_u64_clamp(), 2 << 14);

    let mut n = BigUInt::from(12313137979878979);
    n <<= 3;
    assert_eq!(n.to_u64_clamp(), 12313137979878979 << 3);

    let mut n = BigUInt::from(1233211);
    n <<= 5;
    assert_eq!(n.to_u64_clamp(), 1233211 << 5);

    let mut n = BigUInt::from(1233211);
    n <<= 1;
    assert_eq!(n.to_u64_clamp(), 1233211 << 1);

    let mut n = BigUInt::from(2);
    n <<= 14;
    assert_eq!(n.to_u64_clamp(), 2 << 14);
}

#[test]
fn ops_shift_right() {
    let n = BigUInt::from(1233211123213) >> 5;
    assert_eq!(n.to_u64_clamp(), 1233211123213 >> 5);

    let n = BigUInt::from(1233211998772) >> 1;
    assert_eq!(n.to_u64_clamp(), 1233211998772 >> 1);

    let n = BigUInt::from(2) >> 14;
    assert_eq!(n.to_u64_clamp(), 2 >> 14);

    let mut n = BigUInt::from(12313137979878979);
    n >>= 5;
    assert_eq!(n.to_u64_clamp(), 12313137979878979 >> 5);

    let mut n = BigUInt::from(777777);
    n >>= 1;
    assert_eq!(n.to_u64_clamp(), 777777 >> 1);

    let mut n = BigUInt::from(2);
    n >>= 14;
    assert_eq!(n.to_u64_clamp(), 2 >> 14);
}

#[test]
fn ops_add() {
    let a = BigUInt::from(123213213131231);
    let b = BigUInt::from(56756776529);
    assert_eq!((a + b).to_u64_clamp(), 123213213131231 + 56756776529);

    let a = BigUInt::from(9999977799);
    let b = BigUInt::from(1111111111);
    assert_eq!((a + b).to_u64_clamp(), 9999977799 + 1111111111);

    let a = BigUInt::from(987987987987);
    let b = BigUInt::from(123123123123123132);
    assert_eq!((a + b).to_u64_clamp(), 987987987987 + 123123123123123132);

    let mut n = BigUInt::from(999999);
    n += BigUInt::from(123213123123);
    assert_eq!(n.to_u64_clamp(), 999999 + 123213123123);

    let mut n = BigUInt::from(999999779);
    n += BigUInt::from(66658879);
    assert_eq!(n.to_u64_clamp(), 999999779 + 66658879);

    let mut n = BigUInt::from(789987789);
    n += BigUInt::from(123213123123);
    assert_eq!(n.to_u64_clamp(), 789987789 + 123213123123);
}

#[test]
fn ops_add_u32() {
    let n = BigUInt::from(123213213131231) + 45671;
    assert_eq!(n.to_u64_clamp(), 123213213131231 + 45671);

    let n = BigUInt::from(9999977799) + 11111;
    assert_eq!(n.to_u64_clamp(), 9999977799 + 11111);

    let n = BigUInt::from(987987987987) + 123123;
    assert_eq!(n.to_u64_clamp(), 987987987987 + 123123);

    let mut n = BigUInt::from(999999);
    n += 123123;
    assert_eq!(n.to_u64_clamp(), 999999 + 123123);

    let mut n = BigUInt::from(999999779);
    n += 66658879;
    assert_eq!(n.to_u64_clamp(), 999999779 + 66658879);

    let mut n = BigUInt::from(789987789);
    n += 9999999;
    assert_eq!(n.to_u64_clamp(), 789987789 + 9999999);
}

#[test]
fn ops_mult_u32() {
    let n = BigUInt::from(123213213131231) * 123;
    assert_eq!(n.to_u64_clamp(), 123213213131231 * 123);

    let n = BigUInt::from(9999977799) * 11111;
    assert_eq!(n.to_u64_clamp(), 9999977799 * 11111);

    let n = BigUInt::from(987987987987) * 123123;
    assert_eq!(n.to_u64_clamp(), 987987987987 * 123123);

    let mut n = BigUInt::from(999999);
    n *= 123123;
    assert_eq!(n.to_u64_clamp(), 999999 * 123123);

    let mut n = BigUInt::from(999999779);
    n *= 7788;
    assert_eq!(n.to_u64_clamp(), 999999779 * 7788);

    let mut n = BigUInt::from(789987789);
    n *= 11132;
    assert_eq!(n.to_u64_clamp(), 789987789 * 11132);
}

#[test]
fn from_str_radix() {
    assert_eq!(BigUInt::from_str_radix("123213213131231", 10).unwrap().to_u64_clamp(), 123213213131231);
    assert_eq!(
       BigUInt::from_str_radix("10011101110111111100111111110011100100111", 2).unwrap().to_u64_clamp(),
       0b10011101110111111100111111110011100100111
    );
    assert_eq!(
        BigUInt::from_str_radix("123af81a8d97e6b", 16).unwrap().to_u64_clamp(),
        0x123af81a8d97e6b
    );
    assert_eq!(
        BigUInt::from_str_radix("10736523112", 8).unwrap().to_u64_clamp(),
        0o10736523112
    );
}
