// Answer 0

#[test]
fn test_index_or_insert_with_non_empty_string_and_array() {
    let key = String::from("key");
    let mut value = Value::Array(vec![Value::Bool(true)]);
    key.index_or_insert(&mut value);
}

#[test]
fn test_index_or_insert_with_non_empty_string_and_object() {
    let key = String::from("key");
    let mut value = Value::Object(Map::new());
    key.index_or_insert(&mut value);
}

#[test]
fn test_index_or_insert_with_non_empty_string_and_null() {
    let key = String::from("key");
    let mut value = Value::Null;
    key.index_or_insert(&mut value);
}

