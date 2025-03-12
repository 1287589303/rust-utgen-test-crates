// Answer 0

#[test]
fn test_format_nonfinite_nan() {
    struct TestF64(f64);
    
    let value = TestF64(f64::from_bits(0x7FFFFFFFFFFFFFFF)); // A NaN representation
    assert_eq!(value.0.format_nonfinite(), NAN);
}

#[test]
fn test_format_nonfinite_neg_infinity() {
    struct TestF64(f64);
    
    let value = TestF64(f64::from_bits(0x8000000000000000)); // Negative infinity representation
    assert_eq!(value.0.format_nonfinite(), NEG_INFINITY);
}

#[test]
fn test_format_nonfinite_positive_infinity() {
    struct TestF64(f64);
    
    let value = TestF64(f64::from_bits(0x7F80000000000000)); // Positive infinity representation
    assert_eq!(value.0.format_nonfinite(), INFINITY);
}

