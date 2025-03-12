// Answer 0

#[test]
fn test_decimal_length17_case_1() {
    let v: u64 = 10000000;
    let result = decimal_length17(v);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length17_case_2() {
    let v: u64 = 9999999;
    let result = decimal_length17(v);
    assert_eq!(result, 7);
}

#[test]
fn test_decimal_length17_case_3() {
    let v: u64 = 10;
    let result = decimal_length17(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length17_case_4() {
    let v: u64 = 1;
    let result = decimal_length17(v);
    assert_eq!(result, 1);
}

