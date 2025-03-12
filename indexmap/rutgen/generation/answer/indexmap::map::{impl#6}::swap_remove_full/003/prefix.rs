// Answer 0

#[test]
fn test_swap_remove_full_empty() {
    let mut map: IndexMap<u32, String, RandomState> = IndexMap::new();
    let key: u32 = 1;
    let result = map.swap_remove_full(&key);
}

#[test]
fn test_swap_remove_full_one_entry_not_equivalent() {
    let mut map: IndexMap<u32, String, RandomState> = IndexMap::new();
    map.insert(2, String::from("value"));
    let key: u32 = 1; // key is not equivalent to the existing key (2)
    let result = map.swap_remove_full(&key);
}

