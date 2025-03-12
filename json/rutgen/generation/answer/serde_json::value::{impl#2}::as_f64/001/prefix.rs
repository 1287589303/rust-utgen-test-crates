// Answer 0

#[test]
fn test_as_f64_null() {
    let value = Value::Null;
    value.as_f64();
}

#[test]
fn test_as_f64_bool_true() {
    let value = Value::Bool(true);
    value.as_f64();
}

#[test]
fn test_as_f64_bool_false() {
    let value = Value::Bool(false);
    value.as_f64();
}

#[test]
fn test_as_f64_string() {
    let value = Value::String(String::from("example"));
    value.as_f64();
}

#[test]
fn test_as_f64_empty_array() {
    let value = Value::Array(Vec::new());
    value.as_f64();
}

#[test]
fn test_as_f64_empty_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    value.as_f64();
}

