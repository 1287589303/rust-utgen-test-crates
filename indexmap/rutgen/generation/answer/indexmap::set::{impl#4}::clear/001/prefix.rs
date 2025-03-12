// Answer 0

#[test]
fn test_clear_empty_index_set() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet::with_capacity_and_hasher(0, ());
    set.clear();
}

#[test]
fn test_clear_single_element_index_set() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet::with_capacity_and_hasher(1, ());
    set.insert(1);
    let capacity_before = set.capacity();
    set.clear();
    assert_eq!(set.capacity(), capacity_before);
}

#[test]
fn test_clear_multiple_elements_index_set() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet::with_capacity_and_hasher(10, ());
    for i in 0..10 {
        set.insert(i);
    }
    let capacity_before = set.capacity();
    set.clear();
    assert_eq!(set.capacity(), capacity_before);
}

#[test]
fn test_clear_large_index_set() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet::with_capacity_and_hasher(1000, ());
    for i in 0..1000 {
        set.insert(i);
    }
    let capacity_before = set.capacity();
    set.clear();
    assert_eq!(set.capacity(), capacity_before);
}

