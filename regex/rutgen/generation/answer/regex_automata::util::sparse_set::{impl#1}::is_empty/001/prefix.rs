// Answer 0

#[test]
fn test_sparse_set_is_empty_when_new() {
    let sparse_set = SparseSet::new(10);
    sparse_set.is_empty();
}

#[test]
fn test_sparse_set_is_empty_after_clear() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(StateID(1));
    sparse_set.clear();
    sparse_set.is_empty();
}

#[test]
fn test_sparse_set_is_not_empty_after_insert() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(StateID(1));
    sparse_set.is_empty();
}

#[test]
fn test_sparse_set_is_empty_after_single_insert_and_clear() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(StateID(1));
    sparse_set.clear();
    sparse_set.is_empty();
}

