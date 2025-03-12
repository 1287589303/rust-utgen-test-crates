// Answer 0

#[test]
fn test_iter_empty_sparse_set() {
    let sparse_set = SparseSet::new(10);
    let iter = sparse_set.iter();
}

#[test]
fn test_iter_non_empty_sparse_set_len_one() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(StateID(0));
    let iter = sparse_set.iter();
}

#[test]
fn test_iter_non_empty_sparse_set_len_multiple() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(StateID(0));
    sparse_set.insert(StateID(1));
    let iter = sparse_set.iter();
}

#[test]
fn test_iter_non_empty_sparse_set_with_capacity() {
    let mut sparse_set = SparseSet::new(5);
    for i in 0..5 {
        sparse_set.insert(StateID(i));
    }
    let iter = sparse_set.iter();
}

