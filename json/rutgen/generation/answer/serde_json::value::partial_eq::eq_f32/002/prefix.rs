// Answer 0

#[test]
fn test_eq_f32_with_positive_integer() {
    let number = Number::from_i128(42).unwrap();
    let value = Value::Number(number);
    let result = eq_f32(&value, 42.0);
}

#[test]
fn test_eq_f32_with_negative_integer() {
    let number = Number::from_i128(-42).unwrap();
    let value = Value::Number(number);
    let result = eq_f32(&value, -42.0);
}

#[test]
fn test_eq_f32_with_zero_integer() {
    let number = Number::from_i128(0).unwrap();
    let value = Value::Number(number);
    let result = eq_f32(&value, 0.0);
}

#[test]
fn test_eq_f32_with_positive_float() {
    let number = Number::from_f64(12.34).unwrap();
    let value = Value::Number(number);
    let result = eq_f32(&value, 12.34);
}

#[test]
fn test_eq_f32_with_negative_float() {
    let number = Number::from_f64(-12.34).unwrap();
    let value = Value::Number(number);
    let result = eq_f32(&value, -12.34);
}

#[test]
fn test_eq_f32_with_zero_float() {
    let number = Number::from_f64(0.0).unwrap();
    let value = Value::Number(number);
    let result = eq_f32(&value, 0.0);
}

#[test]
fn test_eq_f32_with_nan() {
    let number = Number::from_f64(f32::NAN as f64).unwrap();
    let value = Value::Number(number);
    let result = eq_f32(&value, f32::NAN);
}

#[test]
fn test_eq_f32_with_positive_infinity() {
    let number = Number::from_f64(f32::INFINITY as f64).unwrap();
    let value = Value::Number(number);
    let result = eq_f32(&value, f32::INFINITY);
}

#[test]
fn test_eq_f32_with_negative_infinity() {
    let number = Number::from_f64(f32::NEG_INFINITY as f64).unwrap();
    let value = Value::Number(number);
    let result = eq_f32(&value, f32::NEG_INFINITY);
}

