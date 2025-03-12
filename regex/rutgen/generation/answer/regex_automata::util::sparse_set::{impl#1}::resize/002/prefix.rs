// Answer 0

#[test]
#[should_panic]
fn test_resize_capacity_exceeds_limit_1() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(StateID::LIMIT + 1);
}

#[test]
#[should_panic]
fn test_resize_capacity_exceeds_limit_2() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(StateID::LIMIT + 2);
}

#[test]
#[should_panic]
fn test_resize_capacity_exceeds_limit_max_usize() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.resize(usize::MAX);
}

