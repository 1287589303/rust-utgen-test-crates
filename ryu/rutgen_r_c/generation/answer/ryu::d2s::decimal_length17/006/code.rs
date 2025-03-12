// Answer 0

#[test]
fn test_decimal_length17_bound_case() {
    let v: u64 = 100_000_000_000;
    let result = decimal_length17(v);
    assert_eq!(result, 12);
}

#[test]
fn test_decimal_length17_high_cases() {
    let v: u64 = 99_999_999_999;
    let result = decimal_length17(v);
    assert_eq!(result, 11);
}

#[test]
fn test_decimal_length17_mid_cases() {
    let v: u64 = 10_000_000_000;
    let result = decimal_length17(v);
    assert_eq!(result, 11);
}

#[test]
fn test_decimal_length17_low_cases() {
    let v: u64 = 1_000_000;
    let result = decimal_length17(v);
    assert_eq!(result, 7);
}

