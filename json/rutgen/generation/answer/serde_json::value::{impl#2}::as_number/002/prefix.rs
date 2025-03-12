// Answer 0

#[test]
fn test_as_number_positive_integer() {
    let value = serde_json::Value::Number(serde_json::Number::from(42u64));
    let _result = value.as_number();
}

#[test]
fn test_as_number_negative_integer() {
    let value = serde_json::Value::Number(serde_json::Number::from(-42i64));
    let _result = value.as_number();
}

#[test]
fn test_as_number_positive_float() {
    let value = serde_json::Value::Number(serde_json::Number::from_f64(3.14).unwrap());
    let _result = value.as_number();
}

#[test]
fn test_as_number_negative_float() {
    let value = serde_json::Value::Number(serde_json::Number::from_f64(-1.5).unwrap());
    let _result = value.as_number();
}

#[test]
fn test_as_number_zero_integer() {
    let value = serde_json::Value::Number(serde_json::Number::from(0u64));
    let _result = value.as_number();
}

#[test]
fn test_as_number_zero_float() {
    let value = serde_json::Value::Number(serde_json::Number::from_f64(0.0).unwrap());
    let _result = value.as_number();
}

