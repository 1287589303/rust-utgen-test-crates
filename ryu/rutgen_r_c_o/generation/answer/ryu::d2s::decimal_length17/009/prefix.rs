// Answer 0

#[test]
fn test_decimal_length17_case1() {
    let v = 100000000;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case2() {
    let v = 99999999;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case3() {
    let v = 1;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case4() {
    let v = 99999999999999999;
    let _ = decimal_length17(v);
}

#[test]
fn test_decimal_length17_case5() {
    let v = 100000001;
    let _ = decimal_length17(v);
}

