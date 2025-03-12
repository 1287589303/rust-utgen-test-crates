// Answer 0

#[test]
fn test_decimal_length17_case_100() {
    let v: u64 = 100;
    let result = decimal_length17(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length17_case_99() {
    let v: u64 = 99;
    let result = decimal_length17(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length17_case_1() {
    let v: u64 = 1;
    let result = decimal_length17(v);
    assert_eq!(result, 1);
}

