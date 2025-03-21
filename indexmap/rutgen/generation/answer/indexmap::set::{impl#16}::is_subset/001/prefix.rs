// Answer 0

#[test]
fn test_is_subset_equal_length_non_empty() {
    let mut set1 = IndexSet::with_capacity_and_hasher(3, RandomState::new());
    let mut set2 = IndexSet::with_capacity_and_hasher(3, RandomState::new());

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);

    let result = set1.is_subset(&set2);
}

#[test]
fn test_is_subset_equal_length_empty() {
    let set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());
    let set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, RandomState::new());

    let result = set1.is_subset(&set2);
}

#[test]
fn test_is_subset_equal_length_subset_non_empty() {
    let mut set1 = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    let mut set2 = IndexSet::with_capacity_and_hasher(2, RandomState::new());

    set1.insert(1);
    set1.insert(2);
    
    set2.insert(1);
    set2.insert(2);
    
    let result = set1.is_subset(&set2);
}

#[test]
fn test_is_subset_equal_length_partial_subset() {
    let mut set1 = IndexSet::with_capacity_and_hasher(3, RandomState::new());
    let mut set2 = IndexSet::with_capacity_and_hasher(3, RandomState::new());

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);
    
    let result = set1.is_subset(&set2);
}

#[test]
fn test_is_subset_equal_length_elements_in_other() {
    let mut set1 = IndexSet::with_capacity_and_hasher(2, RandomState::new());
    let mut set2 = IndexSet::with_capacity_and_hasher(2, RandomState::new());

    set1.insert(10);
    set1.insert(20);
    
    set2.insert(10);
    set2.insert(20);
    
    let result = set1.is_subset(&set2);
}

