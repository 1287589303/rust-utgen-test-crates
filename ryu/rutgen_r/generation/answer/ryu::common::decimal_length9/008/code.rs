// Answer 0

#[test]
fn test_decimal_length9_v_equals_10() {
    let v: u32 = 10;
    let result = decimal_length9(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length9_v_equals_99() {
    let v: u32 = 99;
    let result = decimal_length9(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length9_v_equals_1() {
    let v: u32 = 1;
    let result = decimal_length9(v);
    assert_eq!(result, 1);
}

#[test]
fn test_decimal_length9_v_equals_9() {
    let v: u32 = 9;
    let result = decimal_length9(v);
    assert_eq!(result, 1);
}

