// Answer 0

#[test]
fn test_len_empty_sparse_set() {
    let sparse_set = SparseSet::new(0);
    let length = sparse_set.len();
}

#[test]
fn test_len_non_empty_sparse_set() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(StateID(1));
    sparse_set.insert(StateID(2));
    let length = sparse_set.len();
}

#[test]
fn test_len_after_clear() {
    let mut sparse_set = SparseSet::new(10);
    sparse_set.insert(StateID(1));
    sparse_set.insert(StateID(2));
    sparse_set.clear();
    let length = sparse_set.len();
}

#[test]
fn test_len_with_capacity() {
    let mut sparse_set = SparseSet::new(5);
    sparse_set.insert(StateID(0));
    sparse_set.insert(StateID(4));
    let length = sparse_set.len();
}

#[test]
fn test_len_with_max_state_id() {
    let mut sparse_set = SparseSet::new(StateID::MAX.0 + 1);
    for i in 0..=StateID::MAX.0 {
        sparse_set.insert(StateID(i));
    }
    let length = sparse_set.len();
}

