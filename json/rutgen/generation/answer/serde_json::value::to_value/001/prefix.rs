// Answer 0

#[test]
fn test_to_value_null() {
    let v = serde_json::to_value(());
}

#[test]
fn test_to_value_bool() {
    let v_true = serde_json::to_value(true);
    let v_false = serde_json::to_value(false);
}

#[test]
fn test_to_value_number() {
    let v_integer = serde_json::to_value(42);
    let v_float = serde_json::to_value(3.14);
}

#[test]
fn test_to_value_string() {
    let v_non_empty_string = serde_json::to_value("Hello, World!");
    let v_empty_string = serde_json::to_value("");
}

#[test]
fn test_to_value_array() {
    let v_non_empty_array = serde_json::to_value(vec![1, 2, 3]);
    let v_empty_array = serde_json::to_value(Vec::<i32>::new());
}

#[test]
fn test_to_value_object() {
    let mut valid_map = std::collections::BTreeMap::new();
    valid_map.insert("key1".to_owned(), serde_json::Value::String("value1".to_owned()));
    valid_map.insert("key2".to_owned(), serde_json::Value::Number(serde_json::Number::from(2)));
    let v_valid_object = serde_json::to_value(valid_map);

    let mut nested_object = std::collections::BTreeMap::new();
    nested_object.insert("nested_key".to_owned(), serde_json::Value::String("nested_value".to_owned()));
    let mut outer_map = std::collections::BTreeMap::new();
    outer_map.insert("outer_key".to_owned(), serde_json::Value::Object(nested_object));
    let v_nested_object = serde_json::to_value(outer_map);
}

#[test]
#[should_panic]
fn test_to_value_non_string_map_keys() {
    let mut invalid_map = std::collections::BTreeMap::new();
    invalid_map.insert(vec![1, 2], "Value"); // keys are not strings
    let _ = serde_json::to_value(invalid_map);
}

#[test]
fn test_to_value_large_number() {
    let large_number = serde_json::to_value(i64::MAX);
}

#[test]
fn test_to_value_small_number() {
    let small_number = serde_json::to_value(i64::MIN);
}

#[test]
fn test_to_value_empty_struct() {
    #[derive(Serialize)]
    struct EmptyStruct;

    let v_empty_struct = serde_json::to_value(EmptyStruct);
}

