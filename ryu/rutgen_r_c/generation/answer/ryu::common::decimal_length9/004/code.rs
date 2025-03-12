// Answer 0

#[test]
fn test_decimal_length9_bound_case_6() {
    let v: u32 = 100000;
    let result = decimal_length9(v);
    assert_eq!(result, 6);
}

