// Answer 0

#[test]
fn test_decimal_length17_case_11() {
    let v: u64 = 10000000000;
    let result = decimal_length17(v);
    assert_eq!(result, 11);
}

#[test]
fn test_decimal_length17_case_12() {
    let v: u64 = 99999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 11);
}

#[test]
fn test_decimal_length17_case_10() {
    let v: u64 = 9999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 10);
}

#[test]
#[should_panic]
fn test_decimal_length17_case_invalid() {
    let v: u64 = 100000000000000000;
    decimal_length17(v);
}

