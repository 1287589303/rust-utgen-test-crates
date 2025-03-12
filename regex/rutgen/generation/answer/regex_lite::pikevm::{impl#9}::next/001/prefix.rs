// Answer 0

#[test]
fn test_sparse_set_iter_next_non_empty() {
    let ids: &[StateID] = &[0, 1, 2];
    let mut iter = SparseSetIter(ids.iter());
    let result1 = iter.next();
    let result2 = iter.next();
    let result3 = iter.next();
}

#[test]
fn test_sparse_set_iter_next_empty() {
    let ids: &[StateID] = &[];
    let mut iter = SparseSetIter(ids.iter());
    let result = iter.next();
}

