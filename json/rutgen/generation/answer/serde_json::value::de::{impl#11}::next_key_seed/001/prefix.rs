// Answer 0

#[test]
fn test_next_key_seed_with_string_value() {
    let key = String::from("key1");
    let value = Value::String(String::from("value1"));
    let map = Map::from_iter(vec![(key.clone(), value.clone())]);
    let mut deserializer = MapDeserializer { iter: map.into_iter(), value: None };
    let seed = MapKeyDeserializer { key: Cow::Owned(key) };
    let result = deserializer.next_key_seed(seed);
    let _ = result.unwrap();
}

#[test]
fn test_next_key_seed_with_bool_value() {
    let key = String::from("key2");
    let value = Value::Bool(true);
    let map = Map::from_iter(vec![(key.clone(), value.clone())]);
    let mut deserializer = MapDeserializer { iter: map.into_iter(), value: None };
    let seed = MapKeyDeserializer { key: Cow::Owned(key) };
    let result = deserializer.next_key_seed(seed);
    let _ = result.unwrap();
}

#[test]
fn test_next_key_seed_with_number_value() {
    let key = String::from("key3");
    let value = Value::Number(Number::from(42));
    let map = Map::from_iter(vec![(key.clone(), value.clone())]);
    let mut deserializer = MapDeserializer { iter: map.into_iter(), value: None };
    let seed = MapKeyDeserializer { key: Cow::Owned(key) };
    let result = deserializer.next_key_seed(seed);
    let _ = result.unwrap();
}

#[test]
fn test_next_key_seed_with_null_value() {
    let key = String::from("key4");
    let value = Value::Null;
    let map = Map::from_iter(vec![(key.clone(), value.clone())]);
    let mut deserializer = MapDeserializer { iter: map.into_iter(), value: None };
    let seed = MapKeyDeserializer { key: Cow::Owned(key) };
    let result = deserializer.next_key_seed(seed);
    let _ = result.unwrap();
}

#[test]
fn test_next_key_seed_with_array_value() {
    let key = String::from("key5");
    let value = Value::Array(vec![Value::String(String::from("item1")), Value::String(String::from("item2"))]);
    let map = Map::from_iter(vec![(key.clone(), value.clone())]);
    let mut deserializer = MapDeserializer { iter: map.into_iter(), value: None };
    let seed = MapKeyDeserializer { key: Cow::Owned(key) };
    let result = deserializer.next_key_seed(seed);
    let _ = result.unwrap();
}

#[test]
fn test_next_key_seed_with_object_value() {
    let key = String::from("key6");
    let inner_map = Map::from_iter(vec![(String::from("inner_key"), Value::Number(Number::from(7)))]);
    let value = Value::Object(inner_map);
    let map = Map::from_iter(vec![(key.clone(), value.clone())]);
    let mut deserializer = MapDeserializer { iter: map.into_iter(), value: None };
    let seed = MapKeyDeserializer { key: Cow::Owned(key) };
    let result = deserializer.next_key_seed(seed);
    let _ = result.unwrap();
}

