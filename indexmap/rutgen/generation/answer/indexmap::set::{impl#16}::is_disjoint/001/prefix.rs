// Answer 0

#[test]
fn test_is_disjoint_empty_sets() {
    let set1: super::IndexSet<i32, std::collections::hash_map::RandomState> = super::IndexSet::with_capacity_and_hasher(0, std::collections::hash_map::RandomState::new());
    let set2: super::IndexSet<i32, std::collections::hash_map::RandomState> = super::IndexSet::with_capacity_and_hasher(0, std::collections::hash_map::RandomState::new());
    set1.is_disjoint(&set2);
}

#[test]
fn test_is_disjoint_sets_of_same_size_no_common_elements() {
    let mut set1: super::IndexSet<i32, std::collections::hash_map::RandomState> = super::IndexSet::with_capacity_and_hasher(1, std::collections::hash_map::RandomState::new());
    let mut set2: super::IndexSet<i32, std::collections::hash_map::RandomState> = super::IndexSet::with_capacity_and_hasher(1, std::collections::hash_map::RandomState::new());
    
    // Add distinct elements
    set1.insert(1);
    set2.insert(2);
    
    set1.is_disjoint(&set2);
}

#[test]
fn test_is_disjoint_sets_of_same_size_no_common_elements_multiple() {
    let mut set1: super::IndexSet<i32, std::collections::hash_map::RandomState> = super::IndexSet::with_capacity_and_hasher(2, std::collections::hash_map::RandomState::new());
    let mut set2: super::IndexSet<i32, std::collections::hash_map::RandomState> = super::IndexSet::with_capacity_and_hasher(2, std::collections::hash_map::RandomState::new());

    // Add distinct elements
    set1.insert(1);
    set1.insert(3);
    set2.insert(2);
    set2.insert(4);
    
    set1.is_disjoint(&set2);
}

#[test]
fn test_is_disjoint_large_sets_of_same_size_no_common_elements() {
    let mut set1: super::IndexSet<i32, std::collections::hash_map::RandomState> = super::IndexSet::with_capacity_and_hasher(1000, std::collections::hash_map::RandomState::new());
    let mut set2: super::IndexSet<i32, std::collections::hash_map::RandomState> = super::IndexSet::with_capacity_and_hasher(1000, std::collections::hash_map::RandomState::new());

    // Add distinct elements
    for i in 0..1000 {
        set1.insert(i);
        set2.insert(i + 1000); // Ensures no overlap
    }

    set1.is_disjoint(&set2);
}

