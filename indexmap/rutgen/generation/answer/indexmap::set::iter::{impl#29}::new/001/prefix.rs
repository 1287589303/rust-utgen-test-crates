// Answer 0

#[test]
fn test_intersection_new_with_capacity_and_hasher() {
    use std::collections::hash_map::RandomState;

    let hash_builder = RandomState::new();
    let mut set = IndexSet::with_capacity_and_hasher(10, hash_builder.clone());
    set.insert(1);
    set.insert(2);

    let mut other = IndexSet::with_capacity_and_hasher(10, hash_builder);
    other.insert(2);
    other.insert(3);

    let intersection = Intersection::new(&set, &other);
}

#[test]
fn test_intersection_new_with_hasher() {
    use std::collections::hash_map::RandomState;

    let hash_builder = RandomState::new();
    let mut set = IndexSet::with_hasher(hash_builder.clone());
    set.insert(1);
    set.insert(2);

    let mut other = IndexSet::with_hasher(hash_builder);
    other.insert(2);
    other.insert(3);

    let intersection = Intersection::new(&set, &other);
}

#[test]
fn test_intersection_new_empty_set() {
    use std::collections::hash_map::RandomState;

    let hash_builder = RandomState::new();
    let mut set = IndexSet::with_capacity_and_hasher(10, hash_builder.clone());
    set.insert(1);
    
    let mut other = IndexSet::with_capacity_and_hasher(10, hash_builder);
    other.insert(1);

    let intersection = Intersection::new(&set, &other);
}

#[test]
fn test_intersection_new_large_set() {
    use std::collections::hash_map::RandomState;

    let hash_builder = RandomState::new();
    let mut set = IndexSet::with_capacity_and_hasher(1000, hash_builder.clone());
    for i in 0..1000 {
        set.insert(i);
    }

    let mut other = IndexSet::with_capacity_and_hasher(1000, hash_builder);
    for i in 500..1500 {
        other.insert(i);
    }

    let intersection = Intersection::new(&set, &other);
}

