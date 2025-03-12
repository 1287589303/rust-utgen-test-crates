// Answer 0

#[test]
fn test_resize_to_zero_capacity() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(0);
}

#[test]
#[should_panic]
fn test_resize_exceeding_limit() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(StateID::LIMIT + 1);
}

#[test]
fn test_resize_to_limit_capacity() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(StateID::LIMIT);
}

#[test]
fn test_resize_to_half_capacity() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(StateID::LIMIT / 2);
}

#[test]
fn test_resize_no_change() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(10);
}

