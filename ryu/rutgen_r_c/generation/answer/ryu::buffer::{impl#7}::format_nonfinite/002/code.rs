// Answer 0

#[test]
fn test_format_nonfinite_negative_infinity() {
    struct TestStruct(f64);
    
    let value = TestStruct(f64::from_bits(0x8000000000000000)); // negative infinity
    assert_eq!(value.0.format_nonfinite(), NEG_INFINITY);
}

#[test]
fn test_format_nonfinite_positive_infinity() {
    struct TestStruct(f64);
    
    let value = TestStruct(f64::from_bits(0x7ff0000000000000)); // positive infinity
    assert_eq!(value.0.format_nonfinite(), INFINITY);
}

#[test]
fn test_format_nonfinite_nan() {
    struct TestStruct(f64);
    
    let value = TestStruct(f64::from_bits(0x7ff8000000000000)); // NaN
    assert_eq!(value.0.format_nonfinite(), NAN);
}

