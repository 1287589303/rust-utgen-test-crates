// Answer 0

#[test]
fn test_binary_search_by_key_with_empty_map() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let result = map.binary_search_by_key(&10, |k, _| *k);
}

#[test]
fn test_binary_search_by_key_with_single_entry() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(5, 100);
    let result = map.binary_search_by_key(&5, |k, _| *k);
    let result_not_found = map.binary_search_by_key(&10, |k, _| *k);
}

#[test]
fn test_binary_search_by_key_with_multiple_entries_sorted() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 100);
    map.insert(3, 200);
    map.insert(5, 300);
    let result = map.binary_search_by_key(&3, |k, _| *k);
    let result_not_found = map.binary_search_by_key(&4, |k, _| *k);
}

#[test]
fn test_binary_search_by_key_with_multiple_entries_unsorted() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(3, 200);
    map.insert(1, 100);
    map.insert(5, 300);
    map.sort_keys();
    let result = map.binary_search_by_key(&3, |k, _| *k);
    let result_not_found = map.binary_search_by_key(&4, |k, _| *k);
}

#[test]
fn test_binary_search_by_key_with_boundaries() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(i32::MIN, 0);
    map.insert(i32::MAX, 100);
    let result_min = map.binary_search_by_key(&i32::MIN, |k, _| *k);
    let result_max = map.binary_search_by_key(&i32::MAX, |k, _| *k);
    let result_middle = map.binary_search_by_key(&0, |k, _| *k);
}

#[test]
fn test_binary_search_by_key_with_equal_values() {
    let mut map: IndexMap<String, i32, RandomState> = IndexMap::new();
    map.insert("apple".to_string(), 1);
    map.insert("banana".to_string(), 2);
    map.insert("banana".to_string(), 3); // duplicate key scenario, testing behavior
    let result = map.binary_search_by_key(&"banana".to_string(), |k, _| k.clone());
}

