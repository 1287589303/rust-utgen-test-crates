// Answer 0

#[test]
fn test_is_nonfinite_positive_infinity() {
    let value: f64 = f64::INFINITY;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_negative_infinity() {
    let value: f64 = f64::NEG_INFINITY;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_nan() {
    let value: f64 = f64::NAN;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_regular_value() {
    let value: f64 = 42.0;
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_zero() {
    let value: f64 = 0.0;
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_negative_zero() {
    let value: f64 = -0.0;
    assert!(!value.is_nonfinite());
}

