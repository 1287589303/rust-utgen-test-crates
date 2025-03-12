// Answer 0

#[test]
fn test_decimal_length17_return_3() {
    let v: u64 = 100;
    let result = decimal_length17(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length17_return_4() {
    let v: u64 = 999;
    let result = decimal_length17(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length17_return_5() {
    let v: u64 = 9999;
    let result = decimal_length17(v);
    assert_eq!(result, 5);
}

#[test]
fn test_decimal_length17_return_6() {
    let v: u64 = 99999;
    let result = decimal_length17(v);
    assert_eq!(result, 6);
}

#[test]
fn test_decimal_length17_return_7() {
    let v: u64 = 999999;
    let result = decimal_length17(v);
    assert_eq!(result, 7);
}

#[test]
fn test_decimal_length17_return_8() {
    let v: u64 = 9999999;
    let result = decimal_length17(v);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length17_return_9() {
    let v: u64 = 99999999;
    let result = decimal_length17(v);
    assert_eq!(result, 9);
}

#[test]
fn test_decimal_length17_return_10() {
    let v: u64 = 999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 10);
}

#[test]
fn test_decimal_length17_return_11() {
    let v: u64 = 9999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 11);
}

#[test]
fn test_decimal_length17_return_12() {
    let v: u64 = 99999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 12);
}

#[test]
fn test_decimal_length17_return_13() {
    let v: u64 = 999999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 13);
}

#[test]
fn test_decimal_length17_return_14() {
    let v: u64 = 9999999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 14);
}

#[test]
fn test_decimal_length17_return_15() {
    let v: u64 = 99999999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 15);
}

#[test]
fn test_decimal_length17_return_16() {
    let v: u64 = 999999999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 16);
}

#[test]
fn test_decimal_length17_return_17() {
    let v: u64 = 9999999999999999;
    let result = decimal_length17(v);
    assert_eq!(result, 17);
}

