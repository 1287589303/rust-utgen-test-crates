// Answer 0

#[test]
fn test_from_zero_number() {
    let n = Number::from(0);
    let x: Value = Value::from(n);
}

#[test]
fn test_from_positive_integer_number() {
    let n = Number::from(42);
    let x: Value = Value::from(n);
}

#[test]
fn test_from_negative_integer_number() {
    let n = Number::from(-42);
    let x: Value = Value::from(n);
}

#[test]
fn test_from_large_positive_number() {
    let n = Number::from(1_000_000);
    let x: Value = Value::from(n);
}

#[test]
fn test_from_large_negative_number() {
    let n = Number::from(-1_000_000);
    let x: Value = Value::from(n);
}

#[test]
fn test_from_small_float_number() {
    let n = Number::from(0.1);
    let x: Value = Value::from(n);
}

#[test]
fn test_from_large_float_number() {
    let n = Number::from(1_000_000.123456);
    let x: Value = Value::from(n);
}

#[test]
fn test_from_negative_float_number() {
    let n = Number::from(-1_000_000.123456);
    let x: Value = Value::from(n);
}

#[test]
#[should_panic]
fn test_from_nan_number() {
    let n = Number::from(f64::NAN);
    let x: Value = Value::from(n);
}

