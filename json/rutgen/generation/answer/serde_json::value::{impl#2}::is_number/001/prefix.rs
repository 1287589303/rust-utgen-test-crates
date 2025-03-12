// Answer 0

#[test]
fn test_is_number_null() {
    let v = Value::Null;
    v.is_number();
}

#[test]
fn test_is_number_bool() {
    let v = Value::Bool(true);
    v.is_number();
}

#[test]
fn test_is_number_string() {
    let v = Value::String(String::from("test"));
    v.is_number();
}

#[test]
fn test_is_number_array() {
    let v = Value::Array(Vec::new());
    v.is_number();
}

#[test]
fn test_is_number_object() {
    let v = Value::Object(Map { map: MapImpl::new() });
    v.is_number();
}

