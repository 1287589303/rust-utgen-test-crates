// Answer 0

#[test]
fn test_is_nonfinite_nan() {
    let value: f64 = f64::NAN;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_infinity() {
    let value: f64 = f64::INFINITY;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_neg_infinity() {
    let value: f64 = f64::NEG_INFINITY;
    assert!(value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_finite() {
    let value: f64 = 1.0;
    assert!(!value.is_nonfinite());
}

#[test]
fn test_is_nonfinite_negative_finite() {
    let value: f64 = -1.0;
    assert!(!value.is_nonfinite());
}

