// Answer 0

#[test]
fn test_is_disjoint_self_larger_with_overlap() {
    struct TestHasher;

    let mut self_set = IndexSet::with_capacity_and_hasher(3, TestHasher);
    let mut other_set = IndexSet::with_capacity_and_hasher(2, TestHasher);
    
    self_set.insert(1);
    self_set.insert(2);
    self_set.insert(3);
    
    other_set.insert(2); // Overlapping element
    
    let _result = self_set.is_disjoint(&other_set);
}

#[test]
fn test_is_disjoint_self_larger_no_overlap() {
    struct TestHasher;

    let mut self_set = IndexSet::with_capacity_and_hasher(3, TestHasher);
    let mut other_set = IndexSet::with_capacity_and_hasher(2, TestHasher);
    
    self_set.insert(1);
    self_set.insert(2);
    self_set.insert(3);
    
    other_set.insert(4);
    other_set.insert(5);
    
    let _result = self_set.is_disjoint(&other_set);
}

#[test]
fn test_is_disjoint_equal_size_with_overlap() {
    struct TestHasher;

    let mut self_set = IndexSet::with_capacity_and_hasher(2, TestHasher);
    let mut other_set = IndexSet::with_capacity_and_hasher(2, TestHasher);
    
    self_set.insert(1);
    self_set.insert(2);
    
    other_set.insert(2); // Overlapping element
    other_set.insert(3);
    
    let _result = self_set.is_disjoint(&other_set);
}

#[test]
fn test_is_disjoint_empty_self() {
    struct TestHasher;

    let mut self_set = IndexSet::with_capacity_and_hasher(0, TestHasher);
    let mut other_set = IndexSet::with_capacity_and_hasher(2, TestHasher);
    
    other_set.insert(1);
    other_set.insert(2);
    
    let _result = self_set.is_disjoint(&other_set);
}

#[test]
fn test_is_disjoint_empty_other() {
    struct TestHasher;

    let mut self_set = IndexSet::with_capacity_and_hasher(2, TestHasher);
    let mut other_set = IndexSet::with_capacity_and_hasher(0, TestHasher);
    
    self_set.insert(1);
    self_set.insert(2);
    
    let _result = self_set.is_disjoint(&other_set);
}

