// Answer 0

#[test]
fn test_decimal_length9_case1() {
    let v: u32 = 10000; // v is not a 10-digit number and meets the preconditions
    let result = decimal_length9(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length9_case2() {
    let v: u32 = 9999; // v is below 10000
    let result = decimal_length9(v);
    assert_eq!(result, 4); // valid boundary condition, expects 4
}

#[test]
fn test_decimal_length9_case3() {
    let v: u32 = 100000; // v is exactly 100000
    let result = decimal_length9(v);
    assert_eq!(result, 6); // valid case, expects 6
}

