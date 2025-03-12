// Answer 0

#[test]
fn test_eq_identity_case() {
    use std::collections::hash_map::RandomState;

    let hasher = RandomState::new();
    let mut set1 = IndexSet::with_capacity_and_hasher(3, hasher.clone());
    let mut set2 = IndexSet::with_capacity_and_hasher(3, hasher);

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);

    set2.insert(1);
    set2.insert(2);
    set2.insert(3);

    let _result = set1.eq(&set2);
}

#[test]
fn test_eq_non_identical_sets_same_length() {
    use std::collections::hash_map::RandomState;

    let hasher = RandomState::new();
    let mut set1 = IndexSet::with_capacity_and_hasher(3, hasher.clone());
    let mut set2 = IndexSet::with_capacity_and_hasher(3, hasher);

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);

    set2.insert(4);
    set2.insert(5);
    set2.insert(6);

    let _result = set1.eq(&set2);
}

#[test]
fn test_eq_empty_sets() {
    use std::collections::hash_map::RandomState;

    let hasher = RandomState::new();
    let set1: IndexSet<i32, _> = IndexSet::with_hasher(hasher.clone());
    let set2: IndexSet<i32, _> = IndexSet::with_hasher(hasher);

    let _result = set1.eq(&set2);
}

#[test]
fn test_eq_sets_with_one_element() {
    use std::collections::hash_map::RandomState;

    let hasher = RandomState::new();
    let mut set1 = IndexSet::with_capacity_and_hasher(1, hasher.clone());
    let mut set2 = IndexSet::with_capacity_and_hasher(1, hasher);

    set1.insert(1);
    set2.insert(1);

    let _result = set1.eq(&set2);
}

#[test]
fn test_eq_sets_with_two_elements() {
    use std::collections::hash_map::RandomState;

    let hasher = RandomState::new();
    let mut set1 = IndexSet::with_capacity_and_hasher(2, hasher.clone());
    let mut set2 = IndexSet::with_capacity_and_hasher(2, hasher);

    set1.insert(1);
    set1.insert(2);

    set2.insert(1);
    set2.insert(2);

    let _result = set1.eq(&set2);
}

