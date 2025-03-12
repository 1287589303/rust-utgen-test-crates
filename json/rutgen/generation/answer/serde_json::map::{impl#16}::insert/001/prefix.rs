// Answer 0

#[test]
fn test_insert_null() {
    let mut map = serde_json::Map::new();
    match map.entry("test_null") {
        Entry::Vacant(vacant) => {
            let value = Value::Null;
            let result = vacant.insert(value);
        }
        Entry::Occupied(_) => unimplemented!(),
    }
}

#[test]
fn test_insert_bool() {
    let mut map = serde_json::Map::new();
    match map.entry("test_bool") {
        Entry::Vacant(vacant) => {
            let value = Value::Bool(true);
            let result = vacant.insert(value);
        }
        Entry::Occupied(_) => unimplemented!(),
    }
}

#[test]
fn test_insert_number() {
    let mut map = serde_json::Map::new();
    match map.entry("test_number") {
        Entry::Vacant(vacant) => {
            let value = Value::Number(Number::from(12.5));
            let result = vacant.insert(value);
        }
        Entry::Occupied(_) => unimplemented!(),
    }
}

#[test]
fn test_insert_string() {
    let mut map = serde_json::Map::new();
    match map.entry("test_string") {
        Entry::Vacant(vacant) => {
            let value = Value::String("test".to_string());
            let result = vacant.insert(value);
        }
        Entry::Occupied(_) => unimplemented!(),
    }
}

#[test]
fn test_insert_array() {
    let mut map = serde_json::Map::new();
    match map.entry("test_array") {
        Entry::Vacant(vacant) => {
            let value = Value::Array(vec![Value::String("a".into())]);
            let result = vacant.insert(value);
        }
        Entry::Occupied(_) => unimplemented!(),
    }
}

#[test]
fn test_insert_object() {
    let mut map = serde_json::Map::new();
    match map.entry("test_object") {
        Entry::Vacant(vacant) => {
            let value = Value::Object(serde_json::Map::new());
            let result = vacant.insert(value);
        }
        Entry::Occupied(_) => unimplemented!(),
    }
}

