// Answer 0

#[test]
fn test_iter_empty() {
    let capacity = 5;
    let mut sparse_set = SparseSet::new(capacity);
    let iter = sparse_set.iter();
}

#[test]
fn test_iter_single_element() {
    let capacity = 5;
    let mut sparse_set = SparseSet::new(capacity);
    let id: StateID = 0; 
    sparse_set.insert(id);
    let iter = sparse_set.iter();
}

#[test]
fn test_iter_multiple_elements() {
    let capacity = 5;
    let mut sparse_set = SparseSet::new(capacity);
    for id in 0..3 {
        sparse_set.insert(id);
    }
    let iter = sparse_set.iter();
}

#[test]
fn test_iter_full_capacity() {
    let capacity = 5;
    let mut sparse_set = SparseSet::new(capacity);
    for id in 0..capacity {
        sparse_set.insert(id);
    }
    let iter = sparse_set.iter();
}

