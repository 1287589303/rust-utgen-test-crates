// Answer 0

#[test]
fn test_decimal_length17_boundary_case() {
    let v: u64 = 100_000_000_000_000; // v is 100000000000000, which is 15 digits
    let result = decimal_length17(v);
    assert_eq!(result, 15);
}

