// Answer 0

#[test]
fn test_decimal_length9_value_0() {
    let v: u32 = 0;
    assert_eq!(decimal_length9(v), 1);
}

#[test]
fn test_decimal_length9_value_1() {
    let v: u32 = 1;
    assert_eq!(decimal_length9(v), 1);
}

#[test]
fn test_decimal_length9_value_5() {
    let v: u32 = 5;
    assert_eq!(decimal_length9(v), 1);
}

#[test]
fn test_decimal_length9_value_9() {
    let v: u32 = 9;
    assert_eq!(decimal_length9(v), 1);
}

