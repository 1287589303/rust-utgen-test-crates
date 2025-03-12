// Answer 0

#[test]
fn test_decimal_length9_case_1() {
    let v = 10000000;
    let result = decimal_length9(v);
}

#[test]
fn test_decimal_length9_case_2() {
    let v = 9999999;
    let result = decimal_length9(v);
}

#[test]
fn test_decimal_length9_case_3() {
    let v = 10000001;
    let result = decimal_length9(v);
}

#[test]
fn test_decimal_length9_case_4() {
    let v = 99999998;
    let result = decimal_length9(v);
}

#[test]
fn test_decimal_length9_case_5() {
    let v = 0;
    let result = decimal_length9(v);
}

