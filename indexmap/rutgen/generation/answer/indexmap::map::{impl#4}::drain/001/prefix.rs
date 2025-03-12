// Answer 0

#[test]
fn test_drain_full_range() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(5, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    let _drain = map.drain(..);
}

#[test]
fn test_drain_empty() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(0, RandomState::new());
    
    let _drain = map.drain(..);
}

#[test]
fn test_drain_zero_length() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(1, RandomState::new());
    map.insert(1, 10);
    
    let _drain = map.drain(0..0);
}

#[test]
fn test_drain_full_single_element() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(1, RandomState::new());
    map.insert(1, 10);
    
    let _drain = map.drain(0..1);
}

#[test]
fn test_drain_to_length() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(3, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    let _drain = map.drain(0..3);
}

#[test]
#[should_panic]
fn test_drain_panic_start_gt_end() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(3, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    let _drain = map.drain(2..1);
}

#[test]
#[should_panic]
fn test_drain_panic_end_gt_length() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(3, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    let _drain = map.drain(0..4);
}

#[test]
#[should_panic]
fn test_drain_panic_start_gt_length() {
    let mut map = IndexMap::<i32, i32, RandomState>::with_capacity_and_hasher(3, RandomState::new());
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    
    let _drain = map.drain(3..3);
}

