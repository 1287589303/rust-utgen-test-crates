// Answer 0

#[test]
fn test_and_modify_with_bool_value() {
    let mut map = serde_json::Map::new();
    map.insert("key1".to_string(), Value::Bool(true));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key1").unwrap() });
    
    entry.and_modify(|e| {
        if let Value::Bool(v) = e {
            *v = false;
        }
    });
}

#[test]
fn test_and_modify_with_number_value() {
    let mut map = serde_json::Map::new();
    map.insert("key2".to_string(), Value::Number(serde_json::Number::from(12)));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key2").unwrap() });
    
    entry.and_modify(|e| {
        if let Value::Number(v) = e {
            *v = serde_json::Number::from(20);
        }
    });
}

#[test]
fn test_and_modify_with_string_value() {
    let mut map = serde_json::Map::new();
    map.insert("key3".to_string(), Value::String("test".to_string()));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key3").unwrap() });
    
    entry.and_modify(|e| {
        if let Value::String(v) = e {
            *v = "modified".to_string();
        }
    });
}

#[test]
fn test_and_modify_with_array_value() {
    let mut map = serde_json::Map::new();
    map.insert("key4".to_string(), Value::Array(vec![Value::Bool(false)]));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key4").unwrap() });
    
    entry.and_modify(|e| {
        if let Value::Array(v) = e {
            v.push(Value::Bool(true));
        }
    });
}

