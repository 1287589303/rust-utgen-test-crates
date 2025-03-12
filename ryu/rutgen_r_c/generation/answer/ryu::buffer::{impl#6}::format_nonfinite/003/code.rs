// Answer 0

#[test]
fn test_format_nonfinite_infinity() {
    struct TestValue(f64);
    
    let value = TestValue(f64::INFINITY);
    assert_eq!(value.0.format_nonfinite(), "inf");
}

#[test]
fn test_format_nonfinite_neg_infinity() {
    struct TestValue(f64);
    
    let value = TestValue(f64::NEG_INFINITY);
    assert_eq!(value.0.format_nonfinite(), "-inf");
}

#[test]
fn test_format_nonfinite_nan() {
    struct TestValue(f64);
    
    let value = TestValue(f64::NAN);
    assert_eq!(value.0.format_nonfinite(), "NaN");
}

