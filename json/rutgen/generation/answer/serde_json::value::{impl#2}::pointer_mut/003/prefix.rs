// Answer 0

#[test]
fn test_pointer_mut_empty_string() {
    let mut value = Value::Object(Map::new());
    value.pointer_mut("").map(|_| ());
}

#[test]
fn test_pointer_mut_invalid_pointer() {
    let mut value = Value::Object(Map::new());
    value.pointer_mut("invalid_pointer").map(|_| ());
}

#[test]
fn test_pointer_mut_invalid_index() {
    let mut value = Value::Array(vec![Value::Null, Value::Null]);
    value.pointer_mut("1/2").map(|_| ());
}

