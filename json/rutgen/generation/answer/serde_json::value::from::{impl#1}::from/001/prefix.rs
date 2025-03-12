// Answer 0

#[test]
fn test_from_null_value() {
    let f: f64 = f64::NAN;
    let _x: Value = Value::from(f);
}

#[test]
fn test_from_positive_infinity() {
    let f: f64 = f64::INFINITY;
    let _x: Value = Value::from(f);
}

#[test]
fn test_from_negative_infinity() {
    let f: f64 = f64::NEG_INFINITY;
    let _x: Value = Value::from(f);
}

#[test]
fn test_from_very_small_positive() {
    let f: f64 = 1e-10;
    let _x: Value = Value::from(f);
}

#[test]
fn test_from_very_small_negative() {
    let f: f64 = -1e-10;
    let _x: Value = Value::from(f);
}

#[test]
fn test_from_large_positive() {
    let f: f64 = f64::MAX;
    let _x: Value = Value::from(f);
}

#[test]
fn test_from_large_negative() {
    let f: f64 = -f64::MAX;
    let _x: Value = Value::from(f);
}

#[test]
fn test_from_zero() {
    let f: f64 = 0.0;
    let _x: Value = Value::from(f);
}

#[test]
fn test_from_negative_zero() {
    let f: f64 = -0.0;
    let _x: Value = Value::from(f);
}

#[test]
fn test_from_typical_value_positive() {
    let f: f64 = 12.34;
    let _x: Value = Value::from(f);
}

#[test]
fn test_from_typical_value_negative() {
    let f: f64 = -12.34;
    let _x: Value = Value::from(f);
}

