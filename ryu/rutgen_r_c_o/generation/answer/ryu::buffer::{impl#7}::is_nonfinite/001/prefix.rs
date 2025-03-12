// Answer 0

#[test]
fn test_is_nonfinite_nan() {
    let value: f64 = f64::NAN;
    value.is_nonfinite();
}

#[test]
fn test_is_nonfinite_positive_infinity() {
    let value: f64 = f64::INFINITY;
    value.is_nonfinite();
}

#[test]
fn test_is_nonfinite_negative_infinity() {
    let value: f64 = f64::NEG_INFINITY;
    value.is_nonfinite();
}

#[test]
fn test_is_nonfinite_normal_value() {
    let value: f64 = 3.14;
    value.is_nonfinite();
}

#[test]
fn test_is_nonfinite_negative_normal_value() {
    let value: f64 = -2.71;
    value.is_nonfinite();
}

#[test]
fn test_is_nonfinite_zero() {
    let value: f64 = 0.0;
    value.is_nonfinite();
}

#[test]
fn test_is_nonfinite_negative_zero() {
    let value: f64 = -0.0;
    value.is_nonfinite();
}

#[test]
fn test_is_nonfinite_largest_value() {
    let value: f64 = f64::MAX;
    value.is_nonfinite();
}

#[test]
fn test_is_nonfinite_smallest_value() {
    let value: f64 = f64::MIN;
    value.is_nonfinite();
}

