// Answer 0

#[test]
fn test_shrink_to_edge_case_zero_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to(0);
}

#[test]
fn test_shrink_to_min_capacity_equal_current_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to(100);
}

#[test]
fn test_shrink_to_min_capacity_greater_than_current_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to(150);
}

#[test]
fn test_shrink_to_non_zero_min_capacity() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to(10);
}

#[test]
fn test_shrink_to_min_capacity_one() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    map.shrink_to(1);
}

