// Answer 0

#[test]
fn test_as_array_with_null() {
    let value = Value::Null;
    value.as_array();
}

#[test]
fn test_as_array_with_bool() {
    let value = Value::Bool(true);
    value.as_array();
}

#[test]
fn test_as_array_with_number() {
    let value = Value::Number(Number { n: 1 });
    value.as_array();
}

#[test]
fn test_as_array_with_string() {
    let value = Value::String(String::from("test"));
    value.as_array();
}

#[test]
fn test_as_array_with_object() {
    let value = Value::Object(Map { map: MapImpl::new() });
    value.as_array();
}

