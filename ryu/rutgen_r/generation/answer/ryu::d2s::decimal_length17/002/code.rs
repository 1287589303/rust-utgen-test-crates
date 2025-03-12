// Answer 0

#[test]
fn test_decimal_length17_bound_case_16() {
    let v: u64 = 1000000000000000; // 1 followed by 15 zeros
    let result = decimal_length17(v);
    assert_eq!(result, 16);
}

