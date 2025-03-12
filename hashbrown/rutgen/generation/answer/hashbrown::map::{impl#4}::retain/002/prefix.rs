// Answer 0

#[test]
fn retain_empty_map() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    map.retain(|&k, _| k % 2 == 0);
}

#[test]
fn retain_all_items() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::new(), Global);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.retain(|&k, _| k % 2 == 1);
}

#[test]
fn retain_multiple_items() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::new(), Global);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.retain(|&k, _| k > 1);
}

#[test]
fn retain_none_items() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::new(), Global);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.retain(|&k, _| k > 10);
}

#[test]
fn retain_some_items() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(5, DefaultHashBuilder::new(), Global);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.retain(|&k, _| k == 2 || k == 3);
}

