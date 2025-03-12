// Answer 0

#[test]
fn test_capacity_empty_sparse_set() {
    let sparse_set = SparseSet::new(0);
    let _capacity = sparse_set.capacity();
}

#[test]
fn test_capacity_small_sparse_set() {
    let mut sparse_set = SparseSet::new(5);
    let _ = sparse_set.insert(1);
    let _ = sparse_set.insert(2);
    let _capacity = sparse_set.capacity();
}

#[test]
fn test_capacity_full_sparse_set() {
    let mut sparse_set = SparseSet::new(10);
    for id in 0..10 {
        let _ = sparse_set.insert(id);
    }
    let _capacity = sparse_set.capacity();
}

#[test]
fn test_capacity_large_sparse_set() {
    let mut sparse_set = SparseSet::new(65535);
    for id in 0..65535 {
        let _ = sparse_set.insert(id);
    }
    let _capacity = sparse_set.capacity();
}

#[test]
fn test_capacity_after_clear() {
    let mut sparse_set = SparseSet::new(10);
    let _ = sparse_set.insert(1);
    sparse_set.clear();
    let _capacity = sparse_set.capacity();
}

