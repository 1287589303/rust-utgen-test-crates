// Answer 0

#[test]
fn test_index_into_with_valid_key_present() {
    struct Key;
    impl Index for Key {}

    let key = Key;
    let mut map = Map::new();
    map.insert("key".to_owned(), Value::String("value".to_owned()));
    let value = Value::Object(map);

    key.index_into(&value);
}

#[test]
fn test_index_into_with_valid_key_not_present() {
    struct Key;
    impl Index for Key {}

    let key = Key;
    let mut map = Map::new();
    map.insert("another_key".to_owned(), Value::String("value".to_owned()));
    let value = Value::Object(map);

    key.index_into(&value);
}

#[test]
fn test_index_into_with_empty_map() {
    struct Key;
    impl Index for Key {}

    let key = Key;
    let value = Value::Object(Map::new());

    key.index_into(&value);
}

#[test]
fn test_index_into_with_different_value_variant() {
    struct Key;
    impl Index for Key {}

    let key = Key;
    let value = Value::Array(vec![Value::String("element".to_owned())]);

    key.index_into(&value);
}

