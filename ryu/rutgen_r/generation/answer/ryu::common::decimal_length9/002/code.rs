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

#[test]
fn test_decimal_length9_case3() {
    let v: u32 = 100000000;
    let result = decimal_length9(v);
    assert_eq!(result, 9);
} 

#[test]
fn test_decimal_length9_case4() {
    let v: u32 = 99999999;
    let result = decimal_length9(v);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length9_case5() {
    let v: u32 = 1;
    let result = decimal_length9(v);
    assert_eq!(result, 1);
}

