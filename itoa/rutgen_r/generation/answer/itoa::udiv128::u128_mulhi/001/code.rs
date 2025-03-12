// Answer 0

#[test]
fn test_u128_mulhi_small_values() {
    let x: u128 = 10;
    let y: u128 = 20;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0); // 10 * 20 = 200, upper 128 bits is 0
}

#[test]
fn test_u128_mulhi_large_values() {
    let x: u128 = 0x0000000100000000; // 2^32
    let y: u128 = 0x0000000200000000; // 2^33
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0x0000000000000002); // 2^32 * 2^33 = 2^65, upper 128 bits = 2^65 >> 64 = 2
}

#[test]
fn test_u128_mulhi_full_range() {
    let x: u128 = u128::MAX;
    let y: u128 = u128::MAX;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0xFFFFFFFFFFFFFFFF); // The computation should yield upper 128 bits as 0xFFFFFFFFFFFFFFFF
}

#[test]
fn test_u128_mulhi_zero_values() {
    let x: u128 = 0;
    let y: u128 = 0;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0); // 0 * 0 = 0, upper 128 bits is 0
}

#[test]
fn test_u128_mulhi_mixed_zero_value() {
    let x: u128 = 0;
    let y: u128 = 20;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0); // 0 * 20 = 0, upper 128 bits is 0
}

#[test]
fn test_u128_mulhi_max_and_zero() {
    let x: u128 = u128::MAX;
    let y: u128 = 0;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0); // MAX * 0 = 0, upper 128 bits is 0
}

