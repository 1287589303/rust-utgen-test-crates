// Answer 0

#[test]
fn test_new_bounded_map_min_capacity() {
    let capacity = 1;
    let map = Utf8BoundedMap::new(capacity);
}

#[test]
fn test_new_bounded_map_medium_capacity() {
    let capacity = 1024;
    let map = Utf8BoundedMap::new(capacity);
}

#[test]
fn test_new_bounded_map_max_capacity() {
    let capacity = usize::MAX;
    let map = Utf8BoundedMap::new(capacity);
}

