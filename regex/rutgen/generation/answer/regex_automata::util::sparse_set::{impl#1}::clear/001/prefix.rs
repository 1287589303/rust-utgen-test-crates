// Answer 0

#[test]
fn test_clear_empty_set() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.clear();
}

#[test]
fn test_clear_non_empty_set() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.insert(StateID(1));
    sparse_set.insert(StateID(2));
    sparse_set.clear();
}

#[test]
fn test_clear_set_at_capacity() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.insert(StateID(0));
    sparse_set.insert(StateID(1));
    sparse_set.insert(StateID(2));
    sparse_set.insert(StateID(3));
    sparse_set.insert(StateID(4));
    sparse_set.clear();
}

#[test]
fn test_clear_already_cleared_set() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.clear();
    sparse_set.clear();
}

