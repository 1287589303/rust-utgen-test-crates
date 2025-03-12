// Answer 0

#[test]
fn test_decimal_length9_case1() {
    let v: u32 = 100000000;
    let result = decimal_length9(v);
    assert_eq!(result, 9);
}

#[test]
fn test_decimal_length9_case2() {
    let v: u32 = 999999999; // Test upper boundary just below the limit
    let result = decimal_length9(v);
    assert_eq!(result, 9);
}

