// Answer 0

#[test]
fn test_is_f64_with_null() {
    let value = Value::Null;
    value.is_f64();
}

#[test]
fn test_is_f64_with_bool() {
    let value = Value::Bool(true);
    value.is_f64();
}

#[test]
fn test_is_f64_with_string() {
    let value = Value::String(String::from("test"));
    value.is_f64();
}

#[test]
fn test_is_f64_with_array() {
    let value = Value::Array(Vec::new());
    value.is_f64();
}

#[test]
fn test_is_f64_with_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    value.is_f64();
}

