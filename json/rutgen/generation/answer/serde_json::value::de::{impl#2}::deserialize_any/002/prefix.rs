// Answer 0

#[test]
fn test_deserialize_any_with_non_empty_input() {
    let mut map = Map::new();
    map.insert("key1".to_owned(), Value::Bool(true));
    map.insert("key2".to_owned(), Value::Number(Number::from(42)));

    let visitor = MockVisitor::new(Ok(Value::Object(map.clone())));
    let result = map.deserialize_any(visitor);

    assert!(result.is_ok());
}

#[test]
fn test_deserialize_any_with_additional_elements() {
    let mut map = Map::new();
    map.insert("key1".to_owned(), Value::Bool(true));
    map.insert("key2".to_owned(), Value::Number(Number::from(42)));

    let visitor = MockVisitor::new(Ok(Value::Object(map.clone())));
    let result = map.deserialize_any(visitor);

    assert!(result.is_ok());
}

