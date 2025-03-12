// Answer 0

#[test]
fn test_into_iter_empty() {
    let map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(0, Default::default(), Global);
    let iter = map.into_iter();
    let _count: usize = iter.count(); // Consumed to validate iterator behavior
}

#[test]
fn test_into_iter_single_element() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(1, Default::default(), Global);
    map.insert(1, "a");
    let iter = map.into_iter();
    let _count: usize = iter.count(); // Consumed to validate iterator behavior
}

#[test]
fn test_into_iter_multiple_elements() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(3, Default::default(), Global);
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    let iter = map.into_iter();
    let _count: usize = iter.count(); // Consumed to validate iterator behavior
}

#[test]
fn test_into_iter_full_capacity() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(5, Default::default(), Global);
    for i in 0..5 {
        map.insert(i, "test");
    }
    let iter = map.into_iter();
    let _count: usize = iter.count(); // Consumed to validate iterator behavior
}

#[test]
fn test_into_iter_exceeding_capacity() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(10, Default::default(), Global);
    for i in 0..10 {
        map.insert(i, "test");
    }
    let iter = map.into_iter();
    let _count: usize = iter.count(); // Consumed to validate iterator behavior
}

