// Answer 0

#[test]
fn test_split_off_zero() {
    let mut set = IndexSet::<i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    // Add some elements
    set.reserve(5);
    // Split at index 0
    let result = set.split_off(0);
}

#[test]
fn test_split_off_mid() {
    let mut set = IndexSet::<i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    // Add some elements
    set.reserve(5);
    // Split at index 3
    let result = set.split_off(3);
}

#[test]
fn test_split_off_full_length() {
    let mut set = IndexSet::<i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    // Add some elements
    set.reserve(5);
    // Split at index equal to length
    let len = set.len();
    let result = set.split_off(len);
}

#[test]
#[should_panic]
fn test_split_off_out_of_bounds() {
    let mut set = IndexSet::<i32, RandomState>::with_capacity_and_hasher(10, RandomState::new());
    // Add some elements
    set.reserve(5);
    // Attempt split at index greater than length
    let result = set.split_off(6);
}

