// Answer 0

#[test]
fn test_decimal_length17_case_1() {
    let v: u64 = 0; // v < 100000000000000000 and satisfies all preconditions
    assert_eq!(decimal_length17(v), 1);
}

#[test]
fn test_decimal_length17_case_2() {
    let v: u64 = 5; // v < 100000000000000000 and satisfies all preconditions
    assert_eq!(decimal_length17(v), 1);
}

#[test]
fn test_decimal_length17_case_3() {
    let v: u64 = 9; // v < 100000000000000000 and satisfies all preconditions
    assert_eq!(decimal_length17(v), 1);
}

#[test]
fn test_decimal_length17_case_4() {
    let v: u64 = 10; // edge case where length changes
    assert_eq!(decimal_length17(v), 2);
}

#[test]
fn test_decimal_length17_case_5() {
    let v: u64 = 99; // nearing next boundary, but still below it
    assert_eq!(decimal_length17(v), 2);
}

