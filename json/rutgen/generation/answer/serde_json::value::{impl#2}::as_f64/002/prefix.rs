// Answer 0

#[test]
fn test_as_f64_positive_integer() {
    let number_value = Value::Number(Number::from_i64(123).unwrap());
    let result = number_value.as_f64();
}

#[test]
fn test_as_f64_negative_integer() {
    let number_value = Value::Number(Number::from_i64(-123).unwrap());
    let result = number_value.as_f64();
}

#[test]
fn test_as_f64_zero() {
    let number_value = Value::Number(Number::from_i64(0).unwrap());
    let result = number_value.as_f64();
}

#[test]
fn test_as_f64_positive_float() {
    let number_value = Value::Number(Number::from_f64(123.45).unwrap());
    let result = number_value.as_f64();
}

#[test]
fn test_as_f64_negative_float() {
    let number_value = Value::Number(Number::from_f64(-123.45).unwrap());
    let result = number_value.as_f64();
}

#[test]
fn test_as_f64_zero_float() {
    let number_value = Value::Number(Number::from_f64(0.0).unwrap());
    let result = number_value.as_f64();
}

#[test]
fn test_as_f64_f64_max() {
    let number_value = Value::Number(Number::from_f64(f64::MAX).unwrap());
    let result = number_value.as_f64();
}

#[test]
fn test_as_f64_f64_min() {
    let number_value = Value::Number(Number::from_f64(f64::MIN).unwrap());
    let result = number_value.as_f64();
}

#[test]
fn test_as_f64_nan() {
    let number_value = Value::Number(Number::from_f64(f64::NAN).unwrap());
    let result = number_value.as_f64();
}

#[test]
fn test_as_f64_infinity() {
    let number_value = Value::Number(Number::from_f64(f64::INFINITY).unwrap());
    let result = number_value.as_f64();
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_as_f64_arbitrary_precision_string() {
    let number_value = Value::Number(Number::from_string_unchecked("12345678901234567890".to_string()));
    let result = number_value.as_f64();
}

#[cfg(feature = "arbitrary_precision")]
#[test]
fn test_as_f64_arbitrary_precision_negative_string() {
    let number_value = Value::Number(Number::from_string_unchecked("-12345678901234567890".to_string()));
    let result = number_value.as_f64();
}

