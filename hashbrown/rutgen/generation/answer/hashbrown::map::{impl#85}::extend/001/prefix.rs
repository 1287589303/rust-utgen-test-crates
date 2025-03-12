// Answer 0

#[test]
fn test_extend_with_initial_entry() {
    let mut map = HashMap::new();
    let some_iter = [(1, 1)].into_iter();
    map.extend(some_iter);
}

#[test]
fn test_extend_with_multiple_entries() {
    let mut map = HashMap::new();
    let some_iter = [(1, 1), (2, 2)].into_iter();
    map.extend(some_iter);
}

#[test]
fn test_extend_and_replace_value() {
    let mut map = HashMap::new();
    map.insert(1, 100);
    let some_iter = [(1, 1)].into_iter();
    map.extend(some_iter);
}

#[test]
fn test_extend_with_different_keys() {
    let mut map = HashMap::new();
    let some_iter = [(1, 1), (2, 2), (3, 3)].into_iter();
    map.extend(some_iter);
}

#[test]
fn test_extend_with_duplicate_keys() {
    let mut map = HashMap::new();
    let some_iter = [(1, 1), (1, 2)].into_iter();
    map.extend(some_iter);
}

#[test]
fn test_extend_with_various_collection_types() {
    let mut map = HashMap::new();
    let some_vec: Vec<_> = vec![(4, 4), (5, 5)];
    map.extend(some_vec);
    
    let some_arr = [(6, 6), (7, 7)];
    map.extend(some_arr);
}

