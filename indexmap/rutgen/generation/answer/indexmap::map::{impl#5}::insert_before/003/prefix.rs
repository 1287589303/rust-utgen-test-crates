// Answer 0

#[test]
fn test_insert_before_equal_index_occupied_entry() {
    let mut map: indexmap::IndexMap<char, i32> = indexmap::IndexMap::with_hasher(std::collections::hash_map::RandomState::new());
    map.insert('a', 1);
    
    let (index, old) = map.insert_before(0, 'a', 2);
}

#[test]
fn test_insert_before_equal_index_occupied_entry_multiple() {
    let mut map: indexmap::IndexMap<char, i32> = indexmap::IndexMap::with_hasher(std::collections::hash_map::RandomState::new());
    map.insert('a', 1);
    map.insert('b', 2);
    
    let (index, old) = map.insert_before(1, 'b', 3);
}

#[test]
fn test_insert_before_index_equal_entry_index() {
    let mut map: indexmap::IndexMap<char, i32> = indexmap::IndexMap::with_hasher(std::collections::hash_map::RandomState::new());
    map.insert('c', 3);
    map.insert('d', 4);

    // Attempt to insert before the index of an occupied entry
    let (index, old) = map.insert_before(1, 'c', 5);
}

