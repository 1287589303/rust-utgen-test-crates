// Answer 0

#[test]
fn test_decimal_length17_boundary_case() {
    let v: u64 = 10000;
    let result = decimal_length17(v);
    assert_eq!(result, 5);
}

