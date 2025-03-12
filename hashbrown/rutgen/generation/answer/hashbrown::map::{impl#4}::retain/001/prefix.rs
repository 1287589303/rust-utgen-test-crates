// Answer 0

#[test]
fn test_retain_all_true() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), Global);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    map.retain(|_, _| true);
    
    let len = map.len(); // Expect length to be 3
    let keys: Vec<_> = map.keys().collect(); // Expect keys to be [1, 2, 3]
}

#[test]
fn test_retain_all_false() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), Global);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    map.retain(|_, _| false);
    
    let len = map.len(); // Expect length to be 0
}

#[test]
fn test_retain_some_true() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), Global);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    map.retain(|&k, _| k % 2 == 0);
    
    let len = map.len(); // Expect length to be 1
    let keys: Vec<_> = map.keys().collect(); // Expect keys to be [2]
}

#[test]
fn test_retain_mixed() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, DefaultHashBuilder::new(), Global);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.insert(4, 40);
    
    map.retain(|&k, _| k < 3);
    
    let len = map.len(); // Expect length to be 2
    let keys: Vec<_> = map.keys().collect(); // Expect keys to be [1, 2]
}

