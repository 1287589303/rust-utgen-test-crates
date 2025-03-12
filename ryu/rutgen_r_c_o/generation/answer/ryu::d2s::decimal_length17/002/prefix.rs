// Answer 0

#[test]
fn test_decimal_length17_case_16() {
    let v: u64 = 1000000000000000;
    let result = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_15() {
    let v: u64 = 999999999999999;
    let result = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_14() {
    let v: u64 = 10000000000000;
    let result = decimal_length17(v);
}

