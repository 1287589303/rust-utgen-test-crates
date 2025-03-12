// Answer 0

#[test]
fn test_decimal_length17_boundary_case() {
    let v: u64 = 100_000_000_000; // v is < 100000000000000000 and >= 100000000000
    assert_eq!(decimal_length17(v), 12);
}

