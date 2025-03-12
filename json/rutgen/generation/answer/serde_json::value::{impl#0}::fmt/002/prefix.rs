// Answer 0

#[test]
fn test_fmt_with_empty_object() {
    let value = Value::Object(Map { map: Default::default() });
    let mut formatter = fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_single_entry_object() {
    let mut map = Map { map: Default::default() };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    let value = Value::Object(map);
    let mut formatter = fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_multiple_entries_object() {
    let mut map = Map { map: Default::default() };
    map.map.insert("key1".to_string(), Value::String("value1".to_string()));
    map.map.insert("key2".to_string(), Value::Bool(true));
    let value = Value::Object(map);
    let mut formatter = fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_with_nested_object() {
    let inner_map = Map {
        map: vec![("inner_key".to_string(), Value::Number(Number { n: 42 }))].into_iter().collect(),
    };
    let mut outer_map = Map { map: Default::default() };
    outer_map.map.insert("outer_key".to_string(), Value::Object(inner_map));
    let value = Value::Object(outer_map);
    let mut formatter = fmt::Formatter::new();
    let _ = value.fmt(&mut formatter);
}

