// Answer 0

#[test]
fn test_end_with_non_empty_name_and_non_empty_map() {
    let name = String::from("variant_name");
    let mut map = Map::new();
    map.insert(String::from("key1"), Value::Bool(true));
    let variant = SerializeStructVariant { name, map };
    let result = variant.end();
}

#[test]
fn test_end_with_non_empty_name_and_single_entry_map() {
    let name = String::from("single_entry");
    let mut map = Map::new();
    map.insert(String::from("key1"), Value::Number(Number::from(42)));
    let variant = SerializeStructVariant { name, map };
    let result = variant.end();
}

#[test]
fn test_end_with_non_empty_name_and_multiple_entry_map() {
    let name = String::from("multiple_entries");
    let mut map = Map::new();
    map.insert(String::from("key1"), Value::String(String::from("value1")));
    map.insert(String::from("key2"), Value::Array(vec![Value::Bool(false)]));
    let variant = SerializeStructVariant { name, map };
    let result = variant.end();
}

