// Answer 0

#[test]
fn test_shrink_to_zero_with_elements() {
    let mut set: HashSet<i32> = HashSet::with_capacity(5);
    set.insert(1);
    set.insert(2);
    set.shrink_to(0);
}

#[test]
#[should_panic]
fn test_shrink_to_zero_with_no_elements_panics() {
    let mut set: HashSet<i32> = HashSet::with_capacity(5);
    set.shrink_to(0);
}

#[test]
fn test_shrink_to_equal_current_capacity() {
    let mut set: HashSet<i32> = HashSet::with_capacity(10);
    set.insert(1);
    set.insert(2);
    set.shrink_to(2);
}

#[test]
fn test_shrink_to_below_current_size() {
    let mut set: HashSet<i32> = HashSet::with_capacity(10);
    set.insert(1);
    set.insert(2);
    set.shrink_to(1);
}

#[test]
fn test_shrink_to_with_elements_and_larger_capacity() {
    let mut set: HashSet<i32> = HashSet::with_capacity(100);
    set.insert(1);
    set.insert(2);
    set.shrink_to(50);
}

#[test]
#[should_panic]
fn test_shrink_to_exceeding_current_capacity_panics() {
    let mut set: HashSet<i32> = HashSet::with_capacity(5);
    set.insert(1);
    set.insert(2);
    set.shrink_to(10);
}

