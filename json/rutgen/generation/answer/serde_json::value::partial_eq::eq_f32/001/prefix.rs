// Answer 0

#[test]
fn test_eq_f32_with_null() {
    let value = Value::Null;
    let other = 1.0f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_with_bool() {
    let value = Value::Bool(true);
    let other = 3.14f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_with_string() {
    let value = Value::String("not a number".to_string());
    let other = 2.71f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_with_array() {
    let value = Value::Array(vec![Value::Bool(false), Value::String("test".to_string())]);
    let other = 0.0f32;
    eq_f32(&value, other);
}

#[test]
fn test_eq_f32_with_object() {
    let value = Value::Object(std::collections::BTreeMap::new());
    let other = -1.0f32;
    eq_f32(&value, other);
}

