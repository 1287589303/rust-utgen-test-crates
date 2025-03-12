// Answer 0

#[test]
fn test_format_nonfinite_nan() {
    let value: f64 = f64::from_bits(0x7ff8000000000000); // NaN
    assert_eq!(value.format_nonfinite(), "NaN");
}

#[test]
fn test_format_nonfinite_infinity() {
    let value: f64 = f64::from_bits(0x7ff0000000000000); // +Infinity
    assert_eq!(value.format_nonfinite(), "inf");
}

#[test]
fn test_format_nonfinite_negative_infinity() {
    let value: f64 = f64::from_bits(0xfff0000000000000); // -Infinity
    assert_eq!(value.format_nonfinite(), "-inf");
}

#[test]
fn test_format_nonfinite_finite() {
    let value: f64 = 1.0; // Finite number
    assert_ne!(value.format_nonfinite(), "NaN");
    assert_ne!(value.format_nonfinite(), "inf");
    assert_ne!(value.format_nonfinite(), "-inf");
}

