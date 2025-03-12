// Answer 0

#[should_panic]
fn test_decimal_length9_too_large() {
    decimal_length9(1000000000);
}

