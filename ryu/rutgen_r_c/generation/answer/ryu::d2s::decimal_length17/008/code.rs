// Answer 0

#[test]
fn test_decimal_length17_case_1() {
    let v: u64 = 1000000000;
    let result = decimal_length17(v);
    assert_eq!(result, 10);
}

#[test]
fn test_decimal_length17_case_2() {
    let v: u64 = 999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 9);
}

#[test]
fn test_decimal_length17_case_3() {
    let v: u64 = 100000000;
    let result = decimal_length17(v);
    assert_eq!(result, 9);
}

#[test]
fn test_decimal_length17_case_4() {
    let v: u64 = 10000000;
    let result = decimal_length17(v);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length17_case_5() {
    let v: u64 = 1000000;
    let result = decimal_length17(v);
    assert_eq!(result, 7);
}

#[test]
fn test_decimal_length17_case_6() {
    let v: u64 = 100000;
    let result = decimal_length17(v);
    assert_eq!(result, 6);
}

#[test]
fn test_decimal_length17_case_7() {
    let v: u64 = 10000;
    let result = decimal_length17(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length17_case_8() {
    let v: u64 = 1000;
    let result = decimal_length17(v);
    assert_eq!(result, 4);
}

#[test]
fn test_decimal_length17_case_9() {
    let v: u64 = 100;
    let result = decimal_length17(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length17_case_10() {
    let v: u64 = 10;
    let result = decimal_length17(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length17_case_11() {
    let v: u64 = 1;
    let result = decimal_length17(v);
    assert_eq!(result, 1);
}

