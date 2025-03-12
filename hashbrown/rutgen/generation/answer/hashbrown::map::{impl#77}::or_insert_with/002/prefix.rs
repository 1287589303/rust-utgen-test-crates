// Answer 0

#[test]
fn test_or_insert_with_existing_key() {
    let mut map: hashbrown::HashMap<&str, u32> = hashbrown::HashMap::new();
    map.insert("poneyland", 3);

    let entry = map.entry("poneyland");
    entry.or_insert_with(|| 10);

    let value = entry.into_mut();
    *value *= 2;
}

#[test]
fn test_or_insert_with_occupied_entry() {
    let mut map: hashbrown::HashMap<i32, String> = hashbrown::HashMap::new();
    map.insert(1, "hello".to_string());

    let entry = map.entry(1);
    entry.or_insert_with(|| "world".to_string());

    let value = entry.into_mut();
    value.push_str(", Rust!");
}

