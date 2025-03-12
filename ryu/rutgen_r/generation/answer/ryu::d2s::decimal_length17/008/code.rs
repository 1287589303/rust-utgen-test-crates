// Answer 0

#[test]
fn test_decimal_length17_case_10_digits() {
    let v = 1000000000; // v is exactly 1 billion, which is a 10-digit number
    assert_eq!(ryu::decimal_length17(v), 10);
}

