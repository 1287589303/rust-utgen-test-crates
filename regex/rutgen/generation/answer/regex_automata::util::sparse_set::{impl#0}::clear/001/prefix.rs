// Answer 0

#[test]
fn test_clear_non_empty_sets() {
    let mut sparse_sets = SparseSets::new(10);
    sparse_sets.set1.insert(0);
    sparse_sets.set1.insert(1);
    sparse_sets.set2.insert(2);
    sparse_sets.set2.insert(3);
    sparse_sets.clear();
}

#[test]
fn test_clear_empty_sets() {
    let mut sparse_sets = SparseSets::new(0);
    sparse_sets.clear();
}

#[test]
fn test_clear_with_capacity_one() {
    let mut sparse_sets = SparseSets::new(1);
    sparse_sets.set1.insert(0);
    sparse_sets.clear();
}

#[test]
fn test_clear_with_capacity_ten() {
    let mut sparse_sets = SparseSets::new(10);
    sparse_sets.set1.insert(0);
    sparse_sets.set1.insert(5);
    sparse_sets.set2.insert(1);
    sparse_sets.set2.insert(7);
    sparse_sets.clear();
}

#[test]
#[should_panic]
fn test_clear_on_empty_struct() {
    let mut sparse_sets: SparseSets = SparseSets::new(0);
    sparse_sets.clear();
}

