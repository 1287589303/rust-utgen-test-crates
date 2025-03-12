// Answer 0

#[test]
fn test_is_empty_on_new_index_map() {
    let map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap::with_hasher(std::collections::hash_map::RandomState::new());
    map.is_empty();
}

#[test]
fn test_is_empty_on_empty_index_map() {
    let map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap::with_capacity_and_hasher(0, std::collections::hash_map::RandomState::new());
    map.is_empty();
}

#[test]
fn test_is_empty_on_non_empty_index_map() {
    let mut map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap::with_capacity_and_hasher(1, std::collections::hash_map::RandomState::new());
    map.insert(1, 10);
    map.is_empty();
}

#[test]
fn test_is_empty_on_large_index_map() {
    let mut map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap::with_capacity_and_hasher(100, std::collections::hash_map::RandomState::new());
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    map.is_empty();
} 

#[test]
fn test_is_empty_after_clear() {
    let mut map: super::IndexMap<i32, i32, std::collections::hash_map::RandomState> = super::IndexMap::with_capacity_and_hasher(10, std::collections::hash_map::RandomState::new());
    map.insert(1, 1);
    map.clear();
    map.is_empty();
}

