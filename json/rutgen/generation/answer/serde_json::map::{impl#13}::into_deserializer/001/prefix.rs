// Answer 0

#[test]
fn test_into_deserializer_empty_map() {
    let empty_map: Map<String, Value> = Map { map: MapImpl::new() };
    let _deserializer = empty_map.into_deserializer();
}

#[test]
fn test_into_deserializer_single_entry() {
    let mut single_entry_map: Map<String, Value> = Map { map: MapImpl::new() };
    single_entry_map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    let _deserializer = single_entry_map.into_deserializer();
}

#[test]
fn test_into_deserializer_multiple_entries() {
    let mut multi_entry_map: Map<String, Value> = Map { map: MapImpl::new() };
    for i in 0..10 {
        multi_entry_map.map.insert(format!("key{}", i), Value::Number(Number::from(i)));
    }
    let _deserializer = multi_entry_map.into_deserializer();
}

#[test]
fn test_into_deserializer_max_entries() {
    let mut max_entry_map: Map<String, Value> = Map { map: MapImpl::new() };
    for i in 0..1000 {
        max_entry_map.map.insert(format!("key{}", i), Value::Bool(i % 2 == 0));
    }
    let _deserializer = max_entry_map.into_deserializer();
}

#[test]
fn test_into_deserializer_various_value_types() {
    let mut varied_value_map: Map<String, Value> = Map { map: MapImpl::new() };
    varied_value_map.map.insert("null_value".to_string(), Value::Null);
    varied_value_map.map.insert("bool_value".to_string(), Value::Bool(true));
    varied_value_map.map.insert("number_value".to_string(), Value::Number(Number::from(12.5)));
    varied_value_map.map.insert("string_value".to_string(), Value::String("a string".to_string()));
    varied_value_map.map.insert("array_value".to_string(), Value::Array(vec![
        Value::String("element1".to_string()), 
        Value::String("element2".to_string())
    ]));
    varied_value_map.map.insert("object_value".to_string(), Value::Object(Map {
        map: MapImpl::new(),
    }));
    let _deserializer = varied_value_map.into_deserializer();
}

