// Answer 0

#[test]
fn test_decimal_length17_case_5() {
    let v: u64 = 10000;
    assert_eq!(decimal_length17(v), 5);
}

#[test]
fn test_decimal_length17_case_4() {
    let v: u64 = 9999;
    assert_eq!(decimal_length17(v), 4);
}

#[test]
fn test_decimal_length17_case_3() {
    let v: u64 = 999;
    assert_eq!(decimal_length17(v), 3);
}

#[test]
fn test_decimal_length17_case_2() {
    let v: u64 = 9;
    assert_eq!(decimal_length17(v), 2);
}

#[test]
fn test_decimal_length17_case_1() {
    let v: u64 = 0;
    assert_eq!(decimal_length17(v), 1);
}

