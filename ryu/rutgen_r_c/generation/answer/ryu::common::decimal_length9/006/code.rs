// Answer 0

#[test]
fn test_decimal_length9_case1() {
    let v: u32 = 1000;
    let result = decimal_length9(v);
    assert_eq!(result, 4);
}

