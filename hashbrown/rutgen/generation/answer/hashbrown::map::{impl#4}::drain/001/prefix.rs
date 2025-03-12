// Answer 0

#[test]
fn test_drain_non_empty_hashmap() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map.insert(1, "a");
    map.insert(2, "b");
    let mut drain_iter = map.drain();
    let item = drain_iter.next();
}

#[test]
fn test_drain_empty_hashmap() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    let mut drain_iter = map.drain();
    let item = drain_iter.next();
}

#[test]
fn test_drain_single_item_hashmap() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert(1, "a");
    let mut drain_iter = map.drain();
    let item = drain_iter.next();
}

#[test]
fn test_drain_multiple_items_hashmap() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::new(), Global);
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    map.insert(4, "d");
    let mut drain_iter = map.drain();
    let item1 = drain_iter.next();
    let item2 = drain_iter.next();
}

#[test]
fn test_drain_boundary_capacity_hashmap() {
    let capacity = usize::MAX; // or some reasonable large value for testing
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(capacity, DefaultHashBuilder::new(), Global);
    for i in 0..capacity as i32 {
        map.insert(i, "test");
    }
    let mut drain_iter = map.drain();
    let item = drain_iter.next();
}

