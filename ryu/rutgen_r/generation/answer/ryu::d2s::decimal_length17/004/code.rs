// Answer 0

#[test]
fn test_decimal_length17_case_14() {
    let v: u64 = 10000000000000;
    let result = decimal_length17(v);
    assert_eq!(result, 14);
}

