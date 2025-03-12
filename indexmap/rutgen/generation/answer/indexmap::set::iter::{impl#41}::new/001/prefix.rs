// Answer 0

#[test]
fn test_new_union_with_different_build_hasher() {
    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    let mut set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());

    set1.insert(1);
    set1.insert(2);
    set1.insert(3);

    set2.insert(2);
    set2.insert(3);
    set2.insert(4);

    let union = Union::new(&set1, &set2);
}

#[test]
fn test_new_union_with_custom_build_hasher() {
    use std::collections::hash_map::DefaultHasher;

    let mut set1: IndexSet<i32, DefaultHasher> = IndexSet::with_capacity_and_hasher(10, DefaultHasher::new());
    let mut set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());

    set1.insert(5);
    set1.insert(6);

    set2.insert(6);
    set2.insert(7);
    set2.insert(8);

    let union = Union::new(&set1, &set2);
}

#[test]
fn test_new_union_with_no_common_elements() {
    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    let mut set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());

    set1.insert(1);
    set1.insert(2);

    set2.insert(3);
    set2.insert(4);

    let union = Union::new(&set1, &set2);
}

#[test]
fn test_new_union_with_one_empty_set() {
    use std::collections::hash_map::RandomState;

    let mut set1: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    let mut set2: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, RandomState::new());

    set1.insert(10);
    set1.insert(20);

    let union = Union::new(&set1, &set2);
}

