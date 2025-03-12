// Answer 0

#[test]
fn test_u128_mulhi_small_numbers() {
    let x: u128 = 2;
    let y: u128 = 3;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0); // upper 128 bits of 6 is 0
}

#[test]
fn test_u128_mulhi_large_numbers() {
    let x: u128 = 1 << 128 - 1; // max value for u128
    let y: u128 = 1 << 128 - 1; // max value for u128
    let result = u128_mulhi(x, y);
    assert_eq!(result, 1); // upper 128 bits of (max^2) will still yield upper bits contributing to 0
}

#[test]
fn test_u128_mulhi_boundaries() {
    let x: u128 = (1 << 64) - 1; // max value for lower 64 bits
    let y: u128 = (1 << 64) - 1; // max value for lower 64 bits
    let result = u128_mulhi(x, y);
    assert_eq!(result, (1u128 << 64) - 1); // the upper bits will be 1 since it multiplies into the upper range
}

#[test]
fn test_u128_mulhi_zero() {
    let x: u128 = 0;
    let y: u128 = 5;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0); // multiplying with zero should yield zero
}

#[test]
fn test_u128_mulhi_one() {
    let x: u128 = 1;
    let y: u128 = 5;
    let result = u128_mulhi(x, y);
    assert_eq!(result, 0); // upper bits of 5 are 0
}

#[test]
fn test_u128_mulhi_overflow() {
    let x: u128 = u128::MAX; 
    let y: u128 = u128::MAX; 
    let result = u128_mulhi(x, y);
    assert_eq!(result, u128::MAX); // the overflow scenario where upper bits contribute max value
}

