// Answer 0

#[test]
fn test_decimal_length17_case_14() {
    let v: u64 = 10000000000000;
    let _result = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_14_minus_1() {
    let v: u64 = 9999999999999;
    let _result = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_14_plus_1() {
    let v: u64 = 10000000000001;
    let _result = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_15() {
    let v: u64 = 100000000000000;
    let _result = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_16() {
    let v: u64 = 1000000000000000;
    let _result = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case_17() {
    let v: u64 = 10000000000000000;
    let _result = decimal_length17(v);
}

