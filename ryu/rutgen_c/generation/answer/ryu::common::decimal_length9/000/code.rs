// Answer 0

#[test]
fn test_decimal_length9_single_digit() {
    assert_eq!(decimal_length9(0), 1);
    assert_eq!(decimal_length9(1), 1);
    assert_eq!(decimal_length9(9), 1);
}

#[test]
fn test_decimal_length9_double_digit() {
    assert_eq!(decimal_length9(10), 2);
    assert_eq!(decimal_length9(99), 2);
}

#[test]
fn test_decimal_length9_triple_digit() {
    assert_eq!(decimal_length9(100), 3);
    assert_eq!(decimal_length9(999), 3);
}

#[test]
fn test_decimal_length9_four_digits() {
    assert_eq!(decimal_length9(1000), 4);
    assert_eq!(decimal_length9(9999), 4);
}

#[test]
fn test_decimal_length9_five_digits() {
    assert_eq!(decimal_length9(10000), 5);
    assert_eq!(decimal_length9(99999), 5);
}

#[test]
fn test_decimal_length9_six_digits() {
    assert_eq!(decimal_length9(100000), 6);
    assert_eq!(decimal_length9(999999), 6);
}

#[test]
fn test_decimal_length9_seven_digits() {
    assert_eq!(decimal_length9(1000000), 7);
    assert_eq!(decimal_length9(9999999), 7);
}

#[test]
fn test_decimal_length9_eight_digits() {
    assert_eq!(decimal_length9(10000000), 8);
    assert_eq!(decimal_length9(99999999), 8);
}

#[test]
fn test_decimal_length9_nine_digits() {
    assert_eq!(decimal_length9(100000000), 9);
    assert_eq!(decimal_length9(999999999), 9);
}

