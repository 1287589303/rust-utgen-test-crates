// Answer 0

#[test]
#[should_panic(expected = "assertion failed")]
fn test_decimal_length9_panic_over_10_digits() {
    let v: u32 = 1000000000;
    let _ = decimal_length9(v);
}

