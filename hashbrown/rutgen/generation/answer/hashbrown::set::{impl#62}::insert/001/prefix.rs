// Answer 0

#[test]
fn test_entry_insert_vacant() {
    use hashbrown::hash_map::{Entry, HashMap, DefaultHashBuilder};
    use std::hash::Hash;

    let mut map: HashMap<&str, i32, DefaultHashBuilder> = HashMap::new();
    let entry: Entry<&str, i32> = map.entry("key1").or_insert(0);
    let occupied_entry = entry.insert();
}

#[test]
fn test_entry_insert_vacant_duplicate_key() {
    use hashbrown::hash_map::{Entry, HashMap, DefaultHashBuilder};
    use std::hash::Hash;

    let mut map: HashMap<&str, i32, DefaultHashBuilder> = HashMap::new();
    let entry: Entry<&str, i32> = map.entry("key1").or_insert(0);
    let _ = entry.insert(); // insert first time
    let entry_duplicate: Entry<&str, i32> = map.entry("key1").or_insert(0);
    let occupied_entry_duplicate = entry_duplicate.insert();
}

#[test]
fn test_entry_insert_large_data() {
    use hashbrown::hash_map::{Entry, HashMap, DefaultHashBuilder};
    use std::hash::Hash;

    let mut map: HashMap<i32, i32, DefaultHashBuilder> = HashMap::new();
    for i in 0..1000 {
        let entry: Entry<i32, i32> = map.entry(i);
        let occupied_entry = entry.insert();
    }
}

