// Answer 0

#[test]
fn test_decimal_length9_case_1() {
    let v: u32 = 1000;
    let result = decimal_length9(v);
    assert_eq!(result, 4);
}

#[test]
fn test_decimal_length9_case_2() {
    let v: u32 = 999;
    let result = decimal_length9(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length9_case_3() {
    let v: u32 = 10;
    let result = decimal_length9(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length9_case_4() {
    let v: u32 = 1;
    let result = decimal_length9(v);
    assert_eq!(result, 1);
}

