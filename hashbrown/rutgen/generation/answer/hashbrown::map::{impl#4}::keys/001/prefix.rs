// Answer 0

#[test]
fn test_keys_empty_hashmap() {
    let map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::default(), Global);
    let keys: Keys<_, _, _> = map.keys();
    let vec: Vec<&str> = keys.collect();
}

#[test]
fn test_keys_single_entry() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::default(), Global);
    map.insert("a", 1);
    let keys: Keys<_, _, _> = map.keys();
    let vec: Vec<&str> = keys.collect();
}

#[test]
fn test_keys_multiple_entries() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::default(), Global);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    let keys: Keys<_, _, _> = map.keys();
    let vec: Vec<&str> = keys.collect();
}

#[test]
fn test_keys_order_of_insertion() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher_in(3, DefaultHashBuilder::default(), Global);
    map.insert("b", 2);
    map.insert("a", 1);
    map.insert("c", 3);
    let keys: Keys<_, _, _> = map.keys();
    let mut vec: Vec<&str> = keys.collect();
    vec.sort_unstable();
}

#[test]
fn test_keys_maximum_capacity() {
    let max_capacity = 1000;
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(max_capacity, DefaultHashBuilder::default(), Global);
    for i in 0..max_capacity {
        map.insert(i, i);
    }
    let keys: Keys<_, _, _> = map.keys();
    let vec: Vec<i32> = keys.collect();
}

