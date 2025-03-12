// Answer 0

#[test]
fn test_eq_identical_empty_maps() {
    let map1: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    let map2: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    map1.eq(&map2);
}

#[test]
fn test_eq_identical_non_empty_maps() {
    let mut map1: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    let mut map2: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    
    map1.insert(1, "one");
    map1.insert(2, "two");
    
    map2.insert(1, "one");
    map2.insert(2, "two");
    
    map1.eq(&map2);
}

#[test]
fn test_eq_non_identical_maps() {
    let mut map1: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    let mut map2: HashMap<i32, &str> = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    
    map1.insert(1, "one");
    map1.insert(2, "two");
    
    map2.insert(1, "one");
    map2.insert(3, "three");
    
    map1.eq(&map2);
}

