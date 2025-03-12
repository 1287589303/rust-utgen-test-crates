// Answer 0

#[test]
fn test_decimal_length9_case_100() {
    let v: u32 = 100;
    let result = decimal_length9(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length9_case_99() {
    let v: u32 = 99;
    let result = decimal_length9(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length9_case_1() {
    let v: u32 = 1;
    let result = decimal_length9(v);
    assert_eq!(result, 1);
}

#[test]
fn test_decimal_length9_case_10() {
    let v: u32 = 10;
    let result = decimal_length9(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length9_case_99999() {
    let v: u32 = 99999;
    let result = decimal_length9(v);
    assert_eq!(result, 5);
}

