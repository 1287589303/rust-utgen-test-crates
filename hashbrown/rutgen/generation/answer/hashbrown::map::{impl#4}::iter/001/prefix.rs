// Answer 0

#[test]
fn test_iter_empty_map() {
    let map: HashMap<&str, i32> = HashMap::new();
    let mut iter = map.iter();
    // Calling the method under test
    let _ = iter.next();
}

#[test]
fn test_iter_single_item() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    let mut iter = map.iter();
    // Calling the method under test
    let _ = iter.next();
}

#[test]
fn test_iter_multiple_items() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let mut iter = map.iter();
    // Calling the method under test
    let _ = iter.next();
    let _ = iter.next();
    let _ = iter.next();
}

#[test]
fn test_iter_with_capacity() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::new(), Global);
    for i in 0..5 {
        map.insert(&format!("key{}", i), i as i32);
    }
    let mut iter = map.iter();
    // Calling the method under test
    let _ = iter.next();
    let _ = iter.next();
}

#[test]
fn test_iter_with_max_capacity() {
    const MAX_CAPACITY: usize = 1000;
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(MAX_CAPACITY, DefaultHashBuilder::new(), Global);
    for i in 0..MAX_CAPACITY {
        map.insert(&format!("key{}", i), i as i32);
    }
    let mut iter = map.iter();
    // Calling the method under test
    let _ = iter.next();
}

