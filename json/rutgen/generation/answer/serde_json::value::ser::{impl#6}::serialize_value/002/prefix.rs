// Answer 0

#[test]
fn test_serialize_value_with_string() {
    let mut serialize_map = SerializeMap::Map {
        map: Map::new(),
        next_key: Some("key1".to_string()),
    };
    let value = "test string";
    serialize_map.serialize_value(&value).unwrap();
}

#[test]
fn test_serialize_value_with_number() {
    let mut serialize_map = SerializeMap::Map {
        map: Map::new(),
        next_key: Some("key2".to_string()),
    };
    let value = 42;
    serialize_map.serialize_value(&value).unwrap();
}

#[test]
fn test_serialize_value_with_bool() {
    let mut serialize_map = SerializeMap::Map {
        map: Map::new(),
        next_key: Some("key3".to_string()),
    };
    let value = true;
    serialize_map.serialize_value(&value).unwrap();
}

#[test]
fn test_serialize_value_with_object() {
    let mut serialize_map = SerializeMap::Map {
        map: Map::new(),
        next_key: Some("key4".to_string()),
    };
    let value = Map::new();
    serialize_map.serialize_value(&value).unwrap();
}

#[test]
fn test_serialize_value_with_array() {
    let mut serialize_map = SerializeMap::Map {
        map: Map::new(),
        next_key: Some("key5".to_string()),
    };
    let value = vec![Value::Bool(true), Value::Number(12.5.into())];
    serialize_map.serialize_value(&value).unwrap();
}

