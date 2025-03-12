// Answer 0

#[test]
fn test_sort_keys_empty() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.sort_keys();
}

#[test]
fn test_sort_keys_single_entry() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 100);
    map.sort_keys();
}

#[test]
fn test_sort_keys_multiple_unsorted() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(3, 300);
    map.insert(1, 100);
    map.insert(2, 200);
    map.sort_keys();
}

#[test]
fn test_sort_keys_ascending() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    map.sort_keys();
}

#[test]
fn test_sort_keys_descending() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(3, 300);
    map.insert(2, 200);
    map.insert(1, 100);
    map.sort_keys();
}

#[test]
fn test_sort_keys_mixed() {
    let mut map: IndexMap<String, i32, RandomState> = IndexMap::new();
    map.insert("banana".to_string(), 1);
    map.insert("apple".to_string(), 2);
    map.insert("cherry".to_string(), 3);
    map.sort_keys();
}

