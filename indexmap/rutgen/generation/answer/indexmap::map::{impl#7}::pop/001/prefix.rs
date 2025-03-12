// Answer 0

#[test]
fn test_pop_with_one_element() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    index_map.insert(1, 10);
    assert!(index_map.pop().is_some());
}

#[test]
fn test_pop_with_multiple_elements() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.insert(3, 30);
    assert!(index_map.pop().is_some());
}

#[test]
fn test_pop_with_no_elements() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    assert!(index_map.pop().is_none());
}

#[test]
fn test_pop_until_empty() {
    let mut index_map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    index_map.insert(1, 10);
    index_map.insert(2, 20);
    index_map.pop();
    index_map.pop();
    assert!(index_map.pop().is_none());
}

