// Answer 0

#[test]
fn test_decimal_length9_boundary_case() {
    let v: u32 = 100000;
    let result = decimal_length9(v);
    assert_eq!(result, 6);
}

#[test]
fn test_decimal_length9_one_less_than_boundary() {
    let v: u32 = 99999;
    let result = decimal_length9(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length9_one_more_than_boundary() {
    let v: u32 = 100001;
    let result = decimal_length9(v);
    assert_eq!(result, 6);
}

#[test]
fn test_decimal_length9_max_value() {
    let v: u32 = 999999999; // maximum value under 10 digits
    let result = decimal_length9(v);
    assert_eq!(result, 9);
}

