// Answer 0

#[test]
fn test_pow5factor_32_non_zero_input() {
    let value: u32 = 25; // 25 can be divided by 5 multiple times
    let expected_count: u32 = 2; // 25 -> 5 (1), 5 -> 0 (2)
    let result = pow5factor_32(value);
    assert_eq!(result, expected_count);
}

#[test]
fn test_pow5factor_32_no_factors() {
    let value: u32 = 22; // 22 is not divisible by 5
    let expected_count: u32 = 0; // Loop will break immediately since r != 0
    let result = pow5factor_32(value);
    assert_eq!(result, expected_count);
}

#[test]
fn test_pow5factor_32_edge_case() {
    let value: u32 = 1; // 1 does not have any factors of 5
    let expected_count: u32 = 0; // Loop will break immediately since r != 0
    let result = pow5factor_32(value);
    assert_eq!(result, expected_count);
}

