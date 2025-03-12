// Answer 0

#[test]
fn test_key_occupied_entry_string() {
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("test_key".to_string(), 10);
    let entry = map.entry("test_key".to_string());
    let key = entry.key();
}

#[test]
fn test_key_occupied_entry_integer() {
    let mut map: HashMap<u32, String> = HashMap::new();
    map.insert(1, "one".to_string());
    let entry = map.entry(1);
    let key = entry.key();
}

#[test]
fn test_key_occupied_entry_tuple() {
    let mut map: HashMap<(i32, i32), String> = HashMap::new();
    map.insert((1, 2), "point".to_string());
    let entry = map.entry((1, 2));
    let key = entry.key();
}

