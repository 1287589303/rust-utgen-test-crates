// Answer 0

#[test]
fn test_index_into_mut_with_existing_key() {
    let mut value = Value::Object(Map::from([("key".to_string(), Value::String("value".to_string()))]));
    let key = "key".to_string();
    key.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_non_existing_key() {
    let mut value = Value::Object(Map::from([("key".to_string(), Value::String("value".to_string()))]));
    let key = "non_existing_key".to_string();
    key.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_null_value() {
    let mut value = Value::Object(Map::from([("key".to_string(), Value::Null)]));
    let key = "key".to_string();
    key.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_bool_value() {
    let mut value = Value::Object(Map::from([("key".to_string(), Value::Bool(true))]));
    let key = "key".to_string();
    key.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_number_value() {
    let mut value = Value::Object(Map::from([("key".to_string(), Value::Number(Number::from(42)))]));
    let key = "key".to_string();
    key.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_string_value() {
    let mut value = Value::Object(Map::from([("key".to_string(), Value::String("string_value".to_string()))]));
    let key = "key".to_string();
    key.index_into_mut(&mut value);
}

#[test]
fn test_index_into_mut_with_array_value() {
    let mut value = Value::Object(Map::from([("key".to_string(), Value::Array(vec![Value::String("item".to_string())]))]));
    let key = "key".to_string();
    key.index_into_mut(&mut value);
}

