// Answer 0

#[test]
fn test_decimal_length17_bound_case() {
    let v: u64 = 1000000000000000;
    let result = decimal_length17(v);
    assert_eq!(result, 16);
}

#[test]
fn test_decimal_length17_mid_case() {
    let v: u64 = 1234567890123;
    let result = decimal_length17(v);
    assert_eq!(result, 13);
}

#[test]
fn test_decimal_length17_min_case() {
    let v: u64 = 1;
    let result = decimal_length17(v);
    assert_eq!(result, 1);
}

#[test]
fn test_decimal_length17_max_case() {
    let v: u64 = 99999999999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 17);
}

