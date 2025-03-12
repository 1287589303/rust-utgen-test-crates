// Answer 0

#[test]
fn test_from_finite_positive() {
    let value: Value = from(1.0);
}

#[test]
fn test_from_finite_negative() {
    let value: Value = from(-1.0);
}

#[test]
fn test_from_zero() {
    let value: Value = from(0.0);
}

#[test]
fn test_from_negative_zero() {
    let value: Value = from(-0.0);
}

#[test]
fn test_from_nan() {
    let value: Value = from(f32::NAN);
}

#[test]
fn test_from_infinite_positive() {
    let value: Value = from(f32::INFINITY);
}

#[test]
fn test_from_infinite_negative() {
    let value: Value = from(f32::NEG_INFINITY);
}

#[test]
fn test_from_large_finite() {
    let value: Value = from(3.4e38);
}

#[test]
fn test_from_small_finite() {
    let value: Value = from(-3.4e38);
}

