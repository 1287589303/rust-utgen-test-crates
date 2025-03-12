// Answer 0

#[test]
fn test_decimal_length17_case_1() {
    let v: u64 = 100_000_000; // v >= 100000000
    let result = decimal_length17(v);
    assert_eq!(result, 9);
}

#[test]
fn test_decimal_length17_case_2() {
    let v: u64 = 99_999_999; // v < 100000000
    let result = decimal_length17(v);
    assert_eq!(result, 8);
}

#[test]
fn test_decimal_length17_case_3() {
    let v: u64 = 1_000_000_000; // v >= 1000000000
    let result = decimal_length17(v);
    assert_eq!(result, 10);
}

#[test]
fn test_decimal_length17_case_4() {
    let v: u64 = 10_000_000_000; // v >= 10000000000
    let result = decimal_length17(v);
    assert_eq!(result, 11);
}

#[test]
fn test_decimal_length17_case_5() {
    let v: u64 = 100_000_000_000; // v >= 100000000000
    let result = decimal_length17(v);
    assert_eq!(result, 12);
}

#[test]
fn test_decimal_length17_case_6() {
    let v: u64 = 1_000_000_000_000; // v >= 1000000000000
    let result = decimal_length17(v);
    assert_eq!(result, 13);
}

#[test]
fn test_decimal_length17_case_7() {
    let v: u64 = 10_000_000_000_000; // v >= 10000000000000
    let result = decimal_length17(v);
    assert_eq!(result, 14);
}

#[test]
fn test_decimal_length17_case_8() {
    let v: u64 = 100_000_000_000_000; // v >= 100000000000000
    let result = decimal_length17(v);
    assert_eq!(result, 15);
}

#[test]
fn test_decimal_length17_case_9() {
    let v: u64 = 1_000_000_000_000_000; // v >= 1000000000000000
    let result = decimal_length17(v);
    assert_eq!(result, 16);
}

#[test]
fn test_decimal_length17_case_10() {
    let v: u64 = 10_000_000_000_000_000; // v >= 10000000000000000
    let result = decimal_length17(v);
    assert_eq!(result, 17);
}

