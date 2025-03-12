// Answer 0

#[test]
fn test_decimal_length17_case_1() {
    let value: u64 = 10000000;
    let result = decimal_length17(value);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length17_case_2() {
    let value: u64 = 9999999; // v is less than 10 million
    let result = decimal_length17(value);
    assert_eq!(result, 7);
}

#[test]
fn test_decimal_length17_case_3() {
    let value: u64 = 100000000; // v is exactly 100 million
    let result = decimal_length17(value);
    assert_eq!(result, 9);
}

