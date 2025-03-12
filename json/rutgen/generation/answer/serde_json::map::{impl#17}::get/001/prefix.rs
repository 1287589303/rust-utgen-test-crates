// Answer 0

#[test]
fn test_get_occupied_entry_with_number() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    map.insert("serde".to_owned(), Value::Number(Number::from(12)));
    
    match map.entry("serde") {
        OccupiedEntry { occupied } => {
            let value = occupied.get();
        }
        _ => {}
    }
}

#[test]
fn test_get_occupied_entry_with_boolean() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    map.insert("serde".to_owned(), Value::Bool(true));
    
    match map.entry("serde") {
        OccupiedEntry { occupied } => {
            let value = occupied.get();
        }
        _ => {}
    }
}

#[test]
fn test_get_occupied_entry_with_string() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    map.insert("serde".to_owned(), Value::String("test".to_owned()));
    
    match map.entry("serde") {
        OccupiedEntry { occupied } => {
            let value = occupied.get();
        }
        _ => {}
    }
}

#[test]
fn test_get_occupied_entry_with_array() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    map.insert("serde".to_owned(), Value::Array(vec![Value::String("item".to_owned())]));
    
    match map.entry("serde") {
        OccupiedEntry { occupied } => {
            let value = occupied.get();
        }
        _ => {}
    }
}

#[test]
fn test_get_occupied_entry_with_object() {
    let mut map: MapImpl<String, Value> = MapImpl::new();
    let mut inner_map = MapImpl::new();
    inner_map.insert("key".to_owned(), Value::Number(Number::from(10)));
    map.insert("serde".to_owned(), Value::Object(inner_map));
    
    match map.entry("serde") {
        OccupiedEntry { occupied } => {
            let value = occupied.get();
        }
        _ => {}
    }
}

