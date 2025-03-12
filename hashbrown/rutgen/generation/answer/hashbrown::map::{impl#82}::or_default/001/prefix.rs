// Answer 0

#[test]
fn test_or_default_with_vacant_entry() {
    let mut map: HashMap<String, Option<u32>> = HashMap::new();
    let key = "vacant_key";
    let entry_ref = map.entry_ref(key);
    let value = entry_ref.or_default();
}

#[test]
fn test_or_default_with_vacant_entry_default() {
    let mut map: HashMap<i32, String> = HashMap::new();
    let key = 42;
    let entry_ref = map.entry_ref(&key);
    let value = entry_ref.or_default();
}

#[test]
fn test_or_default_with_vacant_entry_empty_string() {
    let mut map: HashMap<String, String> = HashMap::new();
    let key = "empty_string_key".to_string();
    let entry_ref = map.entry_ref(&key);
    let value = entry_ref.or_default();
}

