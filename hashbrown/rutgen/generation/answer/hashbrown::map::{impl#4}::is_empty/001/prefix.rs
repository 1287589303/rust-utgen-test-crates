// Answer 0

#[test]
fn test_is_empty_initially_empty() {
    let map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    map.is_empty();
}

#[test]
fn test_is_empty_after_insert() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), Global);
    map.insert(1, "a");
    map.is_empty();
}

#[test]
fn test_is_empty_after_clear() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), Global);
    map.insert(1, "a");
    map.clear();
    map.is_empty();
}

#[test]
fn test_is_empty_capacity_zero() {
    let map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    map.is_empty();
}

#[test]
fn test_is_empty_capacity_ten() {
    let mut map: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), Global);
    map.is_empty();
}

