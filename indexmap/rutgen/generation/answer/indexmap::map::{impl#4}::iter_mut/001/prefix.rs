// Answer 0

#[test]
fn test_iter_mut_non_empty() {
    let mut map = IndexMap::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, "one");
    map.insert(2, "two");
    let mut iter = map.iter_mut();
    let (_key1, value1) = iter.next().unwrap();
    let (_key2, value2) = iter.next().unwrap();
    // Iteration logic goes here, using value1 and value2
}

#[test]
fn test_iter_mut_empty() {
    let mut map: IndexMap<i32, &str, RandomState> = IndexMap::with_capacity_and_hasher(0, RandomState::new());
    let iter = map.iter_mut();
    assert!(iter.next().is_none());
}

