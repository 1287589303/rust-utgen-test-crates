// Answer 0

#[test]
fn test_decimal_length17_case_15() {
    let v: u64 = 100_000_000_000_000;
    let result = decimal_length17(v);
    assert_eq!(result, 15);
}

