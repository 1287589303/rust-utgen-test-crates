// Answer 0

#[test]
fn test_binary_search_keys_empty() {
    let map: super::IndexMap<i32, i32> = super::IndexMap::new();
    let result = map.binary_search_keys(&0);
}

#[test]
fn test_binary_search_keys_below_min() {
    let mut map: super::IndexMap<i32, i32> = super::IndexMap::new();
    map.insert(1, 10);
    map.insert(3, 30);
    let result = map.binary_search_keys(&0);
}

#[test]
fn test_binary_search_keys_min() {
    let mut map: super::IndexMap<i32, i32> = super::IndexMap::new();
    map.insert(1, 10);
    map.insert(3, 30);
    let result = map.binary_search_keys(&1);
}

#[test]
fn test_binary_search_keys_between() {
    let mut map: super::IndexMap<i32, i32> = super::IndexMap::new();
    map.insert(1, 10);
    map.insert(3, 30);
    let result = map.binary_search_keys(&2);
}

#[test]
fn test_binary_search_keys_max() {
    let mut map: super::IndexMap<i32, i32> = super::IndexMap::new();
    map.insert(1, 10);
    map.insert(3, 30);
    let result = map.binary_search_keys(&3);
}

#[test]
fn test_binary_search_keys_above_max() {
    let mut map: super::IndexMap<i32, i32> = super::IndexMap::new();
    map.insert(1, 10);
    map.insert(3, 30);
    let result = map.binary_search_keys(&4);
}

