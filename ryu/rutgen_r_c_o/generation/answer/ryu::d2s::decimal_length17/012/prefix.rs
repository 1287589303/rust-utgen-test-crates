// Answer 0

#[test]
fn test_decimal_length17_case1() {
    let v: u64 = 100000;
    let result = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case2() {
    let v: u64 = 99999;
    let result = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case3() {
    let v: u64 = 999999;
    let result = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case4() {
    let v: u64 = 99999999999999999;
    let result = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case5() {
    let v: u64 = 100001;
    let result = decimal_length17(v);
}

