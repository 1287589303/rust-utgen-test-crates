// Answer 0

#[test]
fn test_get_with_existing_key_in_object() {
    let object = Value::Object(Map::new());
    let object_map = object.as_object_mut().unwrap();
    object_map.insert(String::from("A"), Value::Number(Number::from(65)));
    assert!(object.get("A").is_some());
}

#[test]
fn test_get_with_non_existing_key_in_object() {
    let object = Value::Object(Map::new());
    assert!(object.get("B").is_none());
}

#[test]
fn test_get_with_existing_index_in_array() {
    let array = Value::Array(vec![Value::String(String::from("A")), Value::String(String::from("B"))]);
    assert!(array.get(1).is_some());
}

#[test]
fn test_get_with_out_of_bounds_index_in_array() {
    let array = Value::Array(vec![Value::String(String::from("A"))]);
    assert!(array.get(1).is_none());
}

#[test]
fn test_get_with_string_index_on_array() {
    let array = Value::Array(vec![Value::String(String::from("A"))]);
    assert!(array.get("A").is_none());
}

#[test]
fn test_get_with_empty_object() {
    let object = Value::Object(Map::new());
    assert!(object.get("A").is_none());
}

#[test]
fn test_get_with_empty_array() {
    let array = Value::Array(vec![]);
    assert!(array.get(0).is_none());
}

#[test]
fn test_get_with_null_value_in_object() {
    let mut object = Value::Object(Map::new());
    object.as_object_mut().unwrap().insert(String::from("A"), Value::Null);
    assert!(object.get("A").is_some());
}

#[test]
fn test_get_with_null_value_in_array() {
    let mut array = Value::Array(vec![Value::Null]);
    assert!(array.get(0).is_some());
}

