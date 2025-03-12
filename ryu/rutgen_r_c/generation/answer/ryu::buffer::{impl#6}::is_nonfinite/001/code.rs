// Answer 0

#[test]
fn test_is_nonfinite_positive_infinity() {
    let value: f64 = std::f64::INFINITY;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_negative_infinity() {
    let value: f64 = std::f64::NEG_INFINITY;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_nan() {
    let value: f64 = std::f64::NAN;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_normal_value() {
    let value: f64 = 1.0;
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_small_value() {
    let value: f64 = 1e-10;
    assert!(!value.is_nonfinite());
}

