// Answer 0

#[test]
fn test_as_number_with_null() {
    let value = Value::Null;
    value.as_number();
}

#[test]
fn test_as_number_with_bool_true() {
    let value = Value::Bool(true);
    value.as_number();
}

#[test]
fn test_as_number_with_bool_false() {
    let value = Value::Bool(false);
    value.as_number();
}

#[test]
fn test_as_number_with_string() {
    let value = Value::String(String::from("test string"));
    value.as_number();
}

#[test]
fn test_as_number_with_array() {
    let array = vec![Value::Number(Number::from(1)), Value::Number(Number::from(2))];
    let value = Value::Array(array);
    value.as_number();
}

#[test]
fn test_as_number_with_object() {
    let map = Map {
        map: alloc::collections::BTreeMap::new(),
    };
    let value = Value::Object(map);
    value.as_number();
}

