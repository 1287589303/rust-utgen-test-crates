// Answer 0

#[test]
fn test_is_nonfinite_nan() {
    let value: f64 = f64::NAN;
    assert!(value.is_nonfinite());
}

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
fn test_is_nonfinite_finite_value() {
    let value: f64 = 1.0;
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_zero() {
    let value: f64 = 0.0;
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_small_positive() {
    let value: f64 = std::f64::EPSILON; // Small value just above zero
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_small_negative() {
    let value: f64 = -std::f64::EPSILON; // Small value just below zero
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_large_finite_value() {
    let value: f64 = 1e308; // Large finite value
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_just_above_one() {
    let value: f64 = 1.0000001; // Value just above 1.0
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_just_below_one() {
    let value: f64 = 0.9999999; // Value just below 1.0
    assert!(!value.is_nonfinite());
}

