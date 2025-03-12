// Answer 0

#[test]
fn test_or_insert_with_occupied_entry_bool() {
    let mut map = serde_json::Map::new();
    map.insert("test".to_string(), Value::Bool(true));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("test").unwrap() });
    let _ = entry.or_insert_with(|| Value::Bool(false));
}

#[test]
fn test_or_insert_with_occupied_entry_string() {
    let mut map = serde_json::Map::new();
    map.insert("key".to_string(), Value::String("initial".to_string()));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("key").unwrap() });
    let _ = entry.or_insert_with(|| Value::String("new value".to_string()));
}

#[test]
fn test_or_insert_with_occupied_entry_number() {
    let mut map = serde_json::Map::new();
    map.insert("number".to_string(), Value::Number(Number::from(100)));
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("number").unwrap() });
    let _ = entry.or_insert_with(|| Value::Number(Number::from(200)));
}

#[test]
fn test_or_insert_with_occupied_entry_null() {
    let mut map = serde_json::Map::new();
    map.insert("null_key".to_string(), Value::Null);
    let entry = Entry::Occupied(OccupiedEntry { occupied: map.get_mut("null_key").unwrap() });
    let _ = entry.or_insert_with(|| Value::String("should not replace".to_string()));
}

