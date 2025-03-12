// Answer 0

#[test]
fn test_map_ref_deserializer_empty_map() {
    let empty_map = Map { map: MapImpl::new() };
    let deserializer = MapRefDeserializer::new(&empty_map);
}

#[test]
fn test_map_ref_deserializer_single_entry() {
    let single_entry_map = Map { 
        map: MapImpl::from_iter(vec![("key1".to_string(), Value::Null)]) 
    };
    let deserializer = MapRefDeserializer::new(&single_entry_map);
}

#[test]
fn test_map_ref_deserializer_multiple_entries() {
    let multiple_entries_map = Map { 
        map: MapImpl::from_iter(vec![
            ("key1".to_string(), Value::Bool(true)), 
            ("key2".to_string(), Value::Number(Number::from(10))), 
            ("key3".to_string(), Value::String("test".to_string()))
        ]) 
    };
    let deserializer = MapRefDeserializer::new(&multiple_entries_map);
}

#[test]
fn test_map_ref_deserializer_with_array_value() {
    let array_value_map = Map { 
        map: MapImpl::from_iter(vec![
            ("key1".to_string(), Value::Array(vec![Value::String("item1".to_string()), Value::String("item2".to_string())]))
        ]) 
    };
    let deserializer = MapRefDeserializer::new(&array_value_map);
}

#[test]
fn test_map_ref_deserializer_with_object_value() {
    let object_value_map = Map { 
        map: MapImpl::from_iter(vec![
            ("key1".to_string(), Value::Object(Map::from_iter(vec![
                ("nested_key".to_string(), Value::Number(Number::from(2)))
            ])))
        ]) 
    };
    let deserializer = MapRefDeserializer::new(&object_value_map);
}

