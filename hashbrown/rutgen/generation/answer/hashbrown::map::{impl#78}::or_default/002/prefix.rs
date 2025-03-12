// Answer 0

#[test]
fn test_or_default_with_occupied_entry() {
    use hashbrown::hash_map::{Entry, HashMap};
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<&str, Option<u32>> = HashMap::new();
    map.insert("occupied_key", Some(42));

    let entry = map.entry("occupied_key");
    let value = entry.or_default();
    let result = *value;
}

#[test]
fn test_or_default_with_generic_occupied_entry() {
    use hashbrown::hash_map::{Entry, HashMap};
    use std::collections::hash_map::RandomState;

    let mut map: HashMap<u32, String, RandomState> = HashMap::new();
    map.insert(1, String::from("value"));

    let entry = map.entry(1);
    let value = entry.or_default();
    let result = value.clone();
}

