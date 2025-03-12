// Answer 0

#[test]
fn test_is_number_with_integer() {
    let number_value = serde_json::Value::Number(serde_json::Number::from(42));
    assert!(number_value.is_number());
}

#[test]
fn test_is_number_with_float() {
    let number_value = serde_json::Value::Number(serde_json::Number::from_f64(3.14).unwrap());
    assert!(number_value.is_number());
}

#[test]
fn test_is_number_with_large_integer() {
    let number_value = serde_json::Value::Number(serde_json::Number::from(999999999999999999));
    assert!(number_value.is_number());
}

#[test]
fn test_is_number_with_negative_integer() {
    let number_value = serde_json::Value::Number(serde_json::Number::from(-100));
    assert!(number_value.is_number());
}

#[test]
fn test_is_number_with_negative_float() {
    let number_value = serde_json::Value::Number(serde_json::Number::from_f64(-2.718).unwrap());
    assert!(number_value.is_number());
}

