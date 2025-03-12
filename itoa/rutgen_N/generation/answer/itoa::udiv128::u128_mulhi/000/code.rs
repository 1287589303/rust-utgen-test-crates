// Answer 0

#[test]
fn test_u128_mulhi_small_numbers() {
    let x: u128 = 2;
    let y: u128 = 3;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0);
}

#[test]
fn test_u128_mulhi_large_numbers() {
    let x: u128 = 1 << 64; // 2^64
    let y: u128 = 1 << 64; // 2^64
    let result = u128_mulhi(x, y);
    assert_eq!(result, 1);
}

#[test]
fn test_u128_mulhi_mixed_numbers() {
    let x: u128 = 1 << 32; // 2^32
    let y: u128 = 2 << 32; // 2^33
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0);
}

#[test]
fn test_u128_mulhi_boundary_case() {
    let x: u128 = u128::MAX; // max value for u128
    let y: u128 = 1;
    let result = u128_mulhi(x, y);
    assert_eq!(result, u128::MAX >> 64);
}

#[test]
fn test_u128_mulhi_zero() {
    let x: u128 = 0;
    let y: u128 = 0;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0);
}

