// Answer 0

#[test]
fn test_from_key_valid_key() {
    let mut map: IndexMap<String, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    map.insert("key1".to_string(), 1);
    let builder = RawEntryBuilder { map: &map };
    let key = "key1";
    builder.from_key(&key);
}

#[test]
fn test_from_key_non_existent_key() {
    let mut map: IndexMap<String, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    map.insert("key1".to_string(), 1);
    let builder = RawEntryBuilder { map: &map };
    let key = "non_existent_key";
    builder.from_key(&key);
}

#[test]
fn test_from_key_edge_case_null() {
    let mut map: IndexMap<Option<String>, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    map.insert(None, 1);
    let builder = RawEntryBuilder { map: &map };
    let key: Option<String> = None;
    builder.from_key(&key);
}

#[test]
fn test_from_key_at_boundary() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    let builder = RawEntryBuilder { map: &map };
    let key = 9; // Boundary key
    builder.from_key(&key);
}

#[test]
fn test_from_key_empty_map() {
    let map: IndexMap<String, i32, std::collections::hash_map::RandomState> = IndexMap::new();
    let builder = RawEntryBuilder { map: &map };
    let key = "any_key";
    builder.from_key(&key);
}

