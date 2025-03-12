// Answer 0

#[test]
fn test_shift_insert_new_key_at_end() {
    let mut map: IndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    let len = map.len();
    let result = map.shift_insert(len, '+', ());
}

#[test]
fn test_shift_insert_new_key_at_end_with_empty_map() {
    let mut map: IndexMap<char, ()> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let len = map.len();
    let result = map.shift_insert(len, 'a', ());
}

