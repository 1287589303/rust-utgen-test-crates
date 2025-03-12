// Answer 0

#[test]
fn test_index_valid_first_element() {
    let mut index_map = IndexMap::new();
    index_map.insert("key1", "value1");
    let _ = index_map.index(0);
}

#[test]
fn test_index_valid_last_element() {
    let mut index_map = IndexMap::new();
    index_map.insert("key1", "value1");
    index_map.insert("key2", "value2");
    let _ = index_map.index(1);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_invalid_equal_to_len() {
    let mut index_map = IndexMap::new();
    index_map.insert("key1", "value1");
    let _ = index_map.index(1);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_index_invalid_greater_than_len() {
    let mut index_map = IndexMap::new();
    index_map.insert("key1", "value1");
    let _ = index_map.index(2);
}

