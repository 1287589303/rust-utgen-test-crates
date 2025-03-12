// Answer 0

#[test]
#[should_panic]
fn test_insert_before_index_out_of_bounds_negative() {
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let _ = map.insert_before(-1, 3, 30);
}

#[test]
#[should_panic]
fn test_insert_before_index_out_of_bounds_greater_than_len() {
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let len = map.len();
    let _ = map.insert_before(len + 1, 3, 30);
}

