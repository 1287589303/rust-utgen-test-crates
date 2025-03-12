// Answer 0

#[test]
fn test_is_object_with_valid_object() {
    let valid_object = Value::Object(Map { map: MapImpl::new() });
    assert!(valid_object.is_object());
}

#[test]
fn test_is_object_with_valid_object_with_content() {
    let mut contents = Map { map: MapImpl::new() };
    contents.insert("key".to_string(), Value::Number(Number { n: 42 }));
    let valid_object = Value::Object(contents);
    assert!(valid_object.is_object());
}

#[test]
fn test_is_object_with_null() {
    let null_value = Value::Null;
    assert!(!null_value.is_object());
}

#[test]
fn test_is_object_with_boolean_true() {
    let boolean_value = Value::Bool(true);
    assert!(!boolean_value.is_object());
}

#[test]
fn test_is_object_with_boolean_false() {
    let boolean_value = Value::Bool(false);
    assert!(!boolean_value.is_object());
}

#[test]
fn test_is_object_with_number() {
    let number_value = Value::Number(Number { n: 3.14 });
    assert!(!number_value.is_object());
}

#[test]
fn test_is_object_with_string() {
    let string_value = Value::String("Hello, World!".to_string());
    assert!(!string_value.is_object());
}

#[test]
fn test_is_object_with_empty_array() {
    let empty_array = Value::Array(Vec::new());
    assert!(!empty_array.is_object());
}

#[test]
fn test_is_object_with_populated_array() {
    let populated_array = Value::Array(vec![Value::Bool(true), Value::Null]);
    assert!(!populated_array.is_object());
}

