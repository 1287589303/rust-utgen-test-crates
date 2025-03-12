// Answer 0

#[test]
fn test_decimal_length17_edge_case_1000() {
    let v: u64 = 1000;
    let result = decimal_length17(v);
    assert_eq!(result, 4);
}

#[test]
fn test_decimal_length17_edge_case_999() {
    let v: u64 = 999;
    let result = decimal_length17(v);
    assert_eq!(result, 3);
}

#[test]
fn test_decimal_length17_edge_case_10() {
    let v: u64 = 10;
    let result = decimal_length17(v);
    assert_eq!(result, 2);
}

#[test]
fn test_decimal_length17_edge_case_1() {
    let v: u64 = 1;
    let result = decimal_length17(v);
    assert_eq!(result, 1);
}

