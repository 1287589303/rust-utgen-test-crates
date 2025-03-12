// Answer 0

#[test]
fn test_is_empty_on_creation() {
    let set: super::IndexSet<i32, ()> = super::IndexSet::with_capacity_and_hasher(0, ());
    set.is_empty();
}

#[test]
fn test_is_empty_after_adding_elements() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet::with_capacity_and_hasher(0, ());
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.is_empty();
}

#[test]
fn test_is_empty_after_removing_elements() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet::with_capacity_and_hasher(0, ());
    set.map.insert(1, ());
    set.map.remove(&1);
    set.is_empty();
}

#[test]
fn test_is_empty_on_initialization_with_capacity() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet::with_capacity_and_hasher(10, ());
    set.is_empty();
    set.map.insert(1, ());
    set.is_empty();
}

#[test]
fn test_is_empty_on_zero_capacity_set_after_adding() {
    let mut set: super::IndexSet<i32, ()> = super::IndexSet::with_capacity_and_hasher(0, ());
    set.map.insert(1, ());
    set.is_empty();
}

