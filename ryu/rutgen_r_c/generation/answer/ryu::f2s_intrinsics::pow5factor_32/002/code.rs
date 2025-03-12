// Answer 0

#[should_panic]
fn test_pow5factor_32_zero() {
    let value: u32 = 0;
    pow5factor_32(value);
}

#[test]
fn test_pow5factor_32_non_zero() {
    let value: u32 = 125; // 125 = 5^3, so should return 3
    assert_eq!(pow5factor_32(value), 3);
}

#[test]
fn test_pow5factor_32_large_non_zero() {
    let value: u32 = 3125; // 3125 = 5^5, so should return 5
    assert_eq!(pow5factor_32(value), 5);
}

#[test]
fn test_pow5factor_32_no_factor() {
    let value: u32 = 14; // 14 mod 5 != 0, should return 0
    assert_eq!(pow5factor_32(value), 0);
}

