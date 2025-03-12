// Answer 0

#[test]
fn test_pointer_mut_empty_string() {
    let mut value = serde_json::Value::Null;

    let result = value.pointer_mut("");
}

#[test]
fn test_pointer_mut_empty_string_with_object() {
    let mut value = serde_json::Value::Object(serde_json::Map::new());

    let result = value.pointer_mut("");
}

#[test]
fn test_pointer_mut_empty_string_with_array() {
    let mut value = serde_json::Value::Array(vec![]);

    let result = value.pointer_mut("");
}

