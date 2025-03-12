// Answer 0

#[test]
fn test_values_empty() {
    let map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap::with_capacity_and_hasher(0, std::collections::hash_map::RandomState::new());
    let _ = map.values();
}

#[test]
fn test_values_single_entry() {
    let mut map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap::with_capacity_and_hasher(1, std::collections::hash_map::RandomState::new());
    map.insert(1, 10);
    let _ = map.values();
}

#[test]
fn test_values_multiple_entries() {
    let mut map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    for i in 1..=10 {
        map.insert(i, i * 10);
    }
    let _ = map.values();
}

#[test]
fn test_values_large_size() {
    let mut map: super::IndexMap<String, String, std::collections::hash_map::RandomState> = super::IndexMap::with_capacity_and_hasher(1000, std::collections::hash_map::RandomState::new());
    for i in 0..1000 {
        map.insert(format!("key{}", i), format!("value{}", i));
    }
    let _ = map.values();
}

