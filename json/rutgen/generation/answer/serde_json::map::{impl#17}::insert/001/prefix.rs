// Answer 0

#[test]
fn test_insert_null() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let occupied_entry = map.entry("key_null".to_owned()).or_insert(Value::Null);
    let result = occupied_entry.insert(Value::Null);
}

#[test]
fn test_insert_bool_true() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let occupied_entry = map.entry("key_true".to_owned()).or_insert(Value::Bool(false));
    let result = occupied_entry.insert(Value::Bool(true));
}

#[test]
fn test_insert_bool_false() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let occupied_entry = map.entry("key_false".to_owned()).or_insert(Value::Bool(true));
    let result = occupied_entry.insert(Value::Bool(false));
}

#[test]
fn test_insert_number_integer() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let occupied_entry = map.entry("key_integer".to_owned()).or_insert(Value::Number(Number::from(10)));
    let result = occupied_entry.insert(Value::Number(Number::from(20)));
}

#[test]
fn test_insert_number_float() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let occupied_entry = map.entry("key_float".to_owned()).or_insert(Value::Number(Number::from(10.5)));
    let result = occupied_entry.insert(Value::Number(Number::from(20.5)));
}

#[test]
fn test_insert_string_non_empty() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let occupied_entry = map.entry("key_string".to_owned()).or_insert(Value::String("old_value".to_owned()));
    let result = occupied_entry.insert(Value::String("new_value".to_owned()));
}

#[test]
fn test_insert_string_empty() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let occupied_entry = map.entry("key_empty_string".to_owned()).or_insert(Value::String("initial".to_owned()));
    let result = occupied_entry.insert(Value::String("".to_owned()));
}

#[test]
fn test_insert_array_non_empty() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let occupied_entry = map.entry("key_array".to_owned()).or_insert(Value::Array(vec![Value::String("item1".to_owned())]));
    let result = occupied_entry.insert(Value::Array(vec![Value::String("item2".to_owned()), Value::String("item3".to_owned())]));
}

#[test]
fn test_insert_array_empty() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let occupied_entry = map.entry("key_empty_array".to_owned()).or_insert(Value::Array(vec![]));
    let result = occupied_entry.insert(Value::Array(vec![]));
}

#[test]
fn test_insert_object_non_empty() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let inner_map = MapImpl::new();
    let occupied_entry = map.entry("key_object".to_owned()).or_insert(Value::Object(inner_map));
    let result = occupied_entry.insert(Value::Object(MapImpl::from_iter(vec![("inner_key".to_owned(), Value::String("inner_value".to_owned()))])));
}

#[test]
fn test_insert_object_empty() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let occupied_entry = map.entry("key_empty_object".to_owned()).or_insert(Value::Object(MapImpl::new()));
    let result = occupied_entry.insert(Value::Object(MapImpl::new()));
}

