// Answer 0

#[test]
fn test_decimal_length9_with_lower_bound() {
    let v: u32 = 1000000;
    decimal_length9(v);
}

#[test]
fn test_decimal_length9_with_upper_bound() {
    let v: u32 = 999999999;
    decimal_length9(v);
}

