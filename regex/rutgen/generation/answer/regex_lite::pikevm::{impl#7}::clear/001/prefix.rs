// Answer 0

#[test]
fn test_clear_on_empty_set() {
    let mut set = SparseSet {
        len: 0,
        dense: vec![],
        sparse: vec![],
    };
    set.clear();
}

#[test]
fn test_clear_on_non_empty_set() {
    let mut set = SparseSet {
        len: 3,
        dense: vec![1, 2, 3],
        sparse: vec![0, 1, 2],
    };
    set.clear();
}

#[test]
fn test_clear_on_set_with_capacity() {
    let mut set = SparseSet {
        len: 5,
        dense: vec![4, 5, 6, 7, 8],
        sparse: vec![0, 1, 2, 3, 4],
    };
    set.clear();
}

#[test]
fn test_clear_does_not_modify_dense_or_sparse() {
    let mut set = SparseSet {
        len: 2,
        dense: vec![10, 20],
        sparse: vec![1, 0],
    };
    let original_dense = set.dense.clone();
    let original_sparse = set.sparse.clone();
    set.clear();
    assert_eq!(original_dense, set.dense);
    assert_eq!(original_sparse, set.sparse);
}

