// Answer 0

#[test]
fn test_eq_len_not_equal_self_empty_other_non_empty() {
    let map1: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    let map2: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    assert_eq!(map1.eq(&map2), false);
}

#[test]
fn test_eq_len_not_equal_self_non_empty_other_empty() {
    let mut map1 = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map1.insert(1, 100);
    let map2: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(0, DefaultHashBuilder::new(), Global);
    assert_eq!(map1.eq(&map2), false);
}

#[test]
fn test_eq_len_not_equal_self_len_two_other_len_one() {
    let mut map1 = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map1.insert(1, 100);
    map1.insert(2, 200);
    
    let mut map2 = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map2.insert(1, 100);
    
    assert_eq!(map1.eq(&map2), false);
}

#[test]
fn test_eq_len_not_equal_self_len_one_other_len_two() {
    let mut map1 = HashMap::with_capacity_and_hasher_in(1, DefaultHashBuilder::new(), Global);
    map1.insert(1, 100);
    
    let mut map2 = HashMap::with_capacity_and_hasher_in(2, DefaultHashBuilder::new(), Global);
    map2.insert(1, 100);
    map2.insert(2, 200);
    
    assert_eq!(map1.eq(&map2), false);
}

