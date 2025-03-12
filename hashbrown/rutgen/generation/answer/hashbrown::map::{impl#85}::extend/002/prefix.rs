// Answer 0

#[test]
fn test_extend_with_unique_key_value_pairs() {
    let mut map = HashMap::new();
    map.insert(1, 100);
    let some_iter = [(2, 200), (3, 300)].into_iter();
    map.extend(some_iter);
}

#[test]
fn test_extend_with_multiple_unique_key_value_pairs() {
    let mut map = HashMap::new();
    map.insert(1, 100);
    let some_vec: Vec<_> = vec![(2, 200), (3, 300), (4, 400)];
    map.extend(some_vec);
}

#[test]
fn test_extend_with_larger_iterator() {
    let mut map = HashMap::new();
    map.insert(1, 100);
    let some_arr = [(2, 200), (3, 300), (4, 400), (5, 500)];
    map.extend(some_arr);
}

#[test]
fn test_extend_replacing_existing_keys() {
    let mut map = HashMap::new();
    map.insert(1, 100);
    let some_iter = [(1, 150), (2, 200)].into_iter();
    map.extend(some_iter);
    map.insert(3, 300);
}

#[test]
fn test_extend_with_non_empty_map() {
    let mut map = HashMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    let some_vec: Vec<_> = vec![(3, 300), (4, 400)];
    map.extend(some_vec);
}

