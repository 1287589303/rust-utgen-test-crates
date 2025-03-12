// Answer 0

#[test]
fn test_drain_empty_range() {
    let mut set = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.drain(0..0);
}

#[test]
fn test_drain_full_range() {
    let mut set = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.drain(..set.len());
}

#[test]
#[should_panic]
fn test_drain_invalid_start_greater_than_end() {
    let mut set = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.drain(1..0);
}

#[test]
#[should_panic]
fn test_drain_end_greater_than_len() {
    let mut set = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.drain(0..set.len() + 1);
}

#[test]
fn test_drain_single_element() {
    let mut set = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.drain(0..1);
}

#[test]
fn test_drain_subset() {
    let mut set = IndexSet::with_capacity_and_hasher(10, RandomState::new());
    set.drain(2..5);
}

