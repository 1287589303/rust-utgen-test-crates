// Answer 0

#[test]
fn test_pow5factor_32_with_non_zero_value() {
    let value: u32 = 7; // 7 is not a multiple of 5
    let expected_count: u32 = 0; // Since 7 is not a multiple of 5, the count should be 0
    let result = pow5factor_32(value);
    assert_eq!(result, expected_count);
}

#[test]
fn test_pow5factor_32_with_multiple_of_5() {
    let value: u32 = 25; // 25 is 5^2
    let expected_count: u32 = 2; // 25 can be divided by 5 twice before reaching a non-multiple of 5
    let result = pow5factor_32(value);
    assert_eq!(result, expected_count);
}

#[test]
fn test_pow5factor_32_with_large_value() {
    let value: u32 = 125; // 125 is 5^3
    let expected_count: u32 = 3; // 125 can be divided by 5 three times before reaching a non-multiple of 5
    let result = pow5factor_32(value);
    assert_eq!(result, expected_count);
}

#[test]
fn test_pow5factor_32_with_zeroes_in_values() {
    let value: u32 = 100; // 100 is 5^2 * 4
    let expected_count: u32 = 2; // 100 can be divided by 5 two times before reaching a non-multiple of 5
    let result = pow5factor_32(value);
    assert_eq!(result, expected_count);
}

