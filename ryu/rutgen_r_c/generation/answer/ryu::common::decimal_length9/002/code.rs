// Answer 0

#[test]
fn test_decimal_length9_case1() {
    let v: u32 = 10000000;
    let result = decimal_length9(v);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length9_case2() {
    let v: u32 = 9999999;
    let result = decimal_length9(v);
    assert_eq!(result, 7);
}

