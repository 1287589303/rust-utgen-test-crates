// Answer 0

#[test]
fn first_empty_index_map() {
    let index_map: crate::IndexMap<i32, i32, _> = crate::IndexMap::with_hasher(Default::default());
    let result = index_map.first();
}

#[test]
fn first_single_entry_index_map() {
    let mut index_map: crate::IndexMap<i32, i32, _> = crate::IndexMap::with_hasher(Default::default());
    index_map.insert(1, 100);
    let result = index_map.first();
}

#[test]
fn first_multiple_entries_index_map() {
    let mut index_map: crate::IndexMap<i32, i32, _> = crate::IndexMap::with_hasher(Default::default());
    index_map.insert(1, 100);
    index_map.insert(2, 200);
    let result = index_map.first();
}

#[test]
fn first_key_value_pair() {
    let mut index_map: crate::IndexMap<i32, i32, _> = crate::IndexMap::with_hasher(Default::default());
    index_map.insert(10, 1000);
    index_map.insert(20, 2000);
    let result = index_map.first();
}

