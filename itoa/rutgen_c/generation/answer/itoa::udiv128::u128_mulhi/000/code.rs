// Answer 0

#[test]
fn test_u128_mulhi_small_numbers() {
    let result = u128_mulhi(2u128, 3u128);
    assert_eq!(result, 0);
}

#[test]
fn test_u128_mulhi_large_numbers() {
    let result = u128_mulhi(u128::MAX, u128::MAX);
    assert_eq!(result, u128::MAX - 1);
}

#[test]
fn test_u128_mulhi_one_and_zero() {
    let result_zero = u128_mulhi(0u128, 5u128);
    let result_one = u128_mulhi(1u128, 2u128);
    assert_eq!(result_zero, 0);
    assert_eq!(result_one, 0);
}

#[test]
fn test_u128_mulhi_boundary_condition() {
    let result = u128_mulhi(1 << 63, 2u128);
    assert_eq!(result, 1);
}

