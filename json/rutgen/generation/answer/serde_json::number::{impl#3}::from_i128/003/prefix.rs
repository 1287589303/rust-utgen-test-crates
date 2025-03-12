// Answer 0

#[test]
fn test_from_i128_negative_beyond_i64_min() {
    let input = i128::MIN; // this will fail on the u64 test
    let result = Number::from_i128(input);
}

#[test]
fn test_from_i128_positive_beyond_u64_max() {
    let input = 2u128.pow(64) as i128; // this will fail on the u64 test
    let result = Number::from_i128(input);
}

#[test]
fn test_from_i128_invalid_value_near_bounds() {
    let input = i128::MAX; // this will also fail on the u64 test
    let result = Number::from_i128(input);
}

