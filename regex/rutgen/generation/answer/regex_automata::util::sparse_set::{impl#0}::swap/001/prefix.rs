// Answer 0

#[test]
fn test_swap_empty_sets() {
    let mut sparse_sets = SparseSets::new(10);
    sparse_sets.set1 = SparseSet { len: 0, dense: Vec::new(), sparse: Vec::new() };
    sparse_sets.set2 = SparseSet { len: 0, dense: Vec::new(), sparse: Vec::new() };
    sparse_sets.swap();
}

#[test]
fn test_swap_single_element_sets() {
    let mut sparse_sets = SparseSets::new(10);
    sparse_sets.set1 = SparseSet { len: 1, dense: vec![1], sparse: vec![0] };
    sparse_sets.set2 = SparseSet { len: 1, dense: vec![2], sparse: vec![0] };
    sparse_sets.swap();
}

#[test]
fn test_swap_fully_populated_sets() {
    let mut sparse_sets = SparseSets::new(10);
    sparse_sets.set1 = SparseSet { len: 10, dense: (0..10).map(|x| x as StateID).collect(), sparse: (0..10).map(|x| x as StateID).collect() };
    sparse_sets.set2 = SparseSet { len: 10, dense: (10..20).map(|x| x as StateID).collect(), sparse: (0..10).map(|x| (x + 10) as StateID).collect() };
    sparse_sets.swap();
}

#[test]
fn test_swap_with_different_lengths() {
    let mut sparse_sets = SparseSets::new(10);
    sparse_sets.set1 = SparseSet { len: 5, dense: vec![0, 1, 2, 3, 4], sparse: vec![0, 1, 2, 3, 4] };
    sparse_sets.set2 = SparseSet { len: 3, dense: vec![5, 6, 7], sparse: vec![0, 1, 2] };
    sparse_sets.swap();
}

#[test]
fn test_swap_when_one_set_empty() {
    let mut sparse_sets = SparseSets::new(10);
    sparse_sets.set1 = SparseSet { len: 0, dense: Vec::new(), sparse: Vec::new() };
    sparse_sets.set2 = SparseSet { len: 6, dense: vec![0, 1, 2, 3, 4, 5], sparse: vec![0, 1, 2, 3, 4, 5] };
    sparse_sets.swap();
}

