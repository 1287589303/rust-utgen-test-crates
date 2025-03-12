// Answer 0

#[test]
fn test_empty_map() {
    let mut m: Map<String, Value> = Map::new();
    let x: Value = m.into();
}

#[test]
fn test_single_entry_map_with_null() {
    let mut m = Map::new();
    m.insert("key_null".to_owned(), Value::Null);
    let x: Value = m.into();
}

#[test]
fn test_single_entry_map_with_bool() {
    let mut m = Map::new();
    m.insert("key_bool".to_owned(), Value::Bool(true));
    let x: Value = m.into();
}

#[test]
fn test_single_entry_map_with_number() {
    let mut m = Map::new();
    m.insert("key_number".to_owned(), Value::Number(Number { n: 5.into() }));
    let x: Value = m.into();
}

#[test]
fn test_single_entry_map_with_string() {
    let mut m = Map::new();
    m.insert("key_string".to_owned(), Value::String("a string".to_owned()));
    let x: Value = m.into();
}

#[test]
fn test_single_entry_map_with_array() {
    let mut m = Map::new();
    m.insert("key_array".to_owned(), Value::Array(vec![Value::String("element".to_owned())]));
    let x: Value = m.into();
}

#[test]
fn test_multiple_entries_map() {
    let mut m = Map::new();
    m.insert("key1".to_owned(), Value::Bool(false));
    m.insert("key2".to_owned(), Value::Number(Number { n: 3.14.into() }));
    m.insert("key3".to_owned(), Value::String("value".to_owned()));
    let x: Value = m.into();
}

#[test]
fn test_map_with_object() {
    let mut inner_m = Map::new();
    inner_m.insert("inner_key".to_owned(), Value::Number(Number { n: 10.into() }));

    let mut m = Map::new();
    m.insert("outer_key".to_owned(), Value::Object(inner_m));
    let x: Value = m.into();
}

