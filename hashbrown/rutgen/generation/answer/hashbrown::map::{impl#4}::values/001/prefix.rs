// Answer 0

#[test]
fn test_values_with_single_insertion() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map.insert("key1", 42);
    let mut values_iter = map.values();
    let val = values_iter.next().unwrap();
    println!("{}", val);
}

#[test]
fn test_values_with_multiple_insertions() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    map.insert("key1", 42);
    map.insert("key2", 13);
    map.insert("key3", 7);
    let mut values_iter = map.values();
    while let Some(val) = values_iter.next() {
        println!("{}", val);
    }
}

#[test]
fn test_values_empty_map() {
    let map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::new(), Global);
    let values_iter = map.values();
    assert!(values_iter.next().is_none());
}

#[test]
fn test_values_with_integer_keys() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map.insert(1, 100);
    map.insert(2, 200);
    let mut values_iter = map.values();
    while let Some(val) = values_iter.next() {
        println!("{}", val);
    }
}

#[test]
fn test_values_with_exact_capacity() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    let mut values_iter = map.values();
    while let Some(val) = values_iter.next() {
        println!("{}", val);
    }
}

