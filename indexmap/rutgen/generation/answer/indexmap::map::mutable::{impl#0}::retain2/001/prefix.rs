// Answer 0

#[test]
fn test_retain2_empty_index_map() {
    use std::collections::hash_map::RandomState;

    let mut index_map: super::IndexMap<i32, i32, RandomState> = super::IndexMap::new();
    
    index_map.retain2(|_key, _value| true);
}

#[test]
fn test_retain2_single_entry() {
    use std::collections::hash_map::RandomState;

    let mut index_map: super::IndexMap<i32, i32, RandomState> = super::IndexMap::new();
    index_map.insert(1, 10);
    
    index_map.retain2(|key, value| *key % 2 == 0);
}

#[test]
fn test_retain2_multiple_entries() {
    use std::collections::hash_map::RandomState;

    let mut index_map: super::IndexMap<i32, i32, RandomState> = super::IndexMap::new();
    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.insert(3, 30);
    
    index_map.retain2(|key, value| *value > 15);
}

#[test]
fn test_retain2_all_false() {
    use std::collections::hash_map::RandomState;

    let mut index_map: super::IndexMap<i32, i32, RandomState> = super::IndexMap::new();
    index_map.insert(1, 10);
    index_map.insert(2, 20);
    
    index_map.retain2(|_key, _value| false);
}

#[test]
fn test_retain2_all_true() {
    use std::collections::hash_map::RandomState;

    let mut index_map: super::IndexMap<i32, i32, RandomState> = super::IndexMap::new();
    index_map.insert(1, 10);
    index_map.insert(2, 20);
    
    index_map.retain2(|_key, _value| true);
}

