// Answer 0

#[test]
fn test_get_index_entry_out_of_bounds_equal_len() {
    let mut map: crate::IndexMap<u32, u32, std::collections::hash_map::RandomState> = crate::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let len = map.len();
    let result = map.get_index_entry(len);
}

#[test]
fn test_get_index_entry_out_of_bounds_greater_len() {
    let mut map: crate::IndexMap<u32, u32, std::collections::hash_map::RandomState> = crate::IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let len = map.len();
    let result = map.get_index_entry(len + 1);
}

