// Answer 0

#[test]
fn test_decimal_length9_large_value() {
    let value = 999999999;
    let result = decimal_length9(value);
    assert_eq!(result, 9);
}

#[test]
fn test_decimal_length9_med_large_value() {
    let value = 99999999;
    let result = decimal_length9(value);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length9_med_value() {
    let value = 9999999;
    let result = decimal_length9(value);
    assert_eq!(result, 7);
}

#[test]
fn test_decimal_length9_small_value() {
    let value = 99999;
    let result = decimal_length9(value);
    assert_eq!(result, 6);
}

#[test]
fn test_decimal_length9_smallest_value() {
    let value = 1;
    let result = decimal_length9(value);
    assert_eq!(result, 1);
}

#[test]
#[should_panic]
fn test_decimal_length9_ten_digit_value() {
    let value = 1000000000;
    decimal_length9(value);
}

