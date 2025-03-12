// Answer 0

#[test]
fn test_split_off_with_zero() {
    let mut map = IndexMap::<u32, String, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    map.insert(4, "four".to_string());
    let _ = map.split_off(0);
}

#[test]
fn test_split_off_with_length() {
    let mut map = IndexMap::<u32, String, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    let new_map = map.split_off(map.len());
}

#[test]
#[should_panic]
fn test_split_off_panic_on_out_of_bounds() {
    let mut map = IndexMap::<u32, String, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    let _ = map.split_off(3); // This should panic.
}

#[test]
fn test_split_off_with_middle_index() {
    let mut map = IndexMap::<u32, String, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    let new_map = map.split_off(1);
}

