// Answer 0

#[test]
fn test_memory_usage_empty_sets() {
    let sparse_sets = SparseSets::new(0);
    let usage = sparse_sets.memory_usage();
}

#[test]
fn test_memory_usage_single_element_set1() {
    let mut sparse_sets = SparseSets::new(1);
    sparse_sets.set1.insert(0);
    let usage = sparse_sets.memory_usage();
}

#[test]
fn test_memory_usage_single_element_set2() {
    let mut sparse_sets = SparseSets::new(1);
    sparse_sets.set2.insert(0);
    let usage = sparse_sets.memory_usage();
}

#[test]
fn test_memory_usage_multiple_elements_set1() {
    let mut sparse_sets = SparseSets::new(10);
    for i in 0..5 {
        sparse_sets.set1.insert(i);
    }
    let usage = sparse_sets.memory_usage();
}

#[test]
fn test_memory_usage_multiple_elements_set2() {
    let mut sparse_sets = SparseSets::new(10);
    for i in 5..10 {
        sparse_sets.set2.insert(i);
    }
    let usage = sparse_sets.memory_usage();
}

#[test]
fn test_memory_usage_max_capacity() {
    let mut sparse_sets = SparseSets::new(1000);
    for i in 0..1000 {
        sparse_sets.set1.insert(i);
        sparse_sets.set2.insert(i);
    }
    let usage = sparse_sets.memory_usage();
}

