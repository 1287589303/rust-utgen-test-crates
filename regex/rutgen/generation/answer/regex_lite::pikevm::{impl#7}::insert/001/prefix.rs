// Answer 0

#[test]
fn test_insert_return_false_when_id_exists() {
    // Initialize a SparseSet with the capacity of at least 2.
    let mut set = SparseSet::new(2);
    // Insert a StateID into the set.
    let id: StateID = StateID::from(1);
    let _ = set.insert(id); // Insert an ID to ensure it's contained.

    // Now call insert with the same id which should return false.
    let result = set.insert(id);
}

#[test]
fn test_insert_return_false_with_multiple_elements() {
    // Initialize a SparseSet with the capacity of at least 3.
    let mut set = SparseSet::new(3);
    // Insert multiple StateIDs into the set.
    let id1: StateID = StateID::from(2);
    let id2: StateID = StateID::from(3);
    let _ = set.insert(id1);
    let _ = set.insert(id2);

    // Call insert with one of the existing ids which should return false.
    let result = set.insert(id1);
}

#[test]
fn test_insert_return_false_at_boundary_id() {
    // Initialize a SparseSet with capacity sufficient for at least 1 element.
    let mut set = SparseSet::new(1);
    // Insert a StateID at the boundary of valid StateID values.
    let id: StateID = StateID::from(0);
    let _ = set.insert(id); // Insert to ensure it's contained.

    // Call insert with the same id which should return false.
    let result = set.insert(id);
}

#[test]
fn test_insert_return_false_with_max_valid_state_id() {
    // Initialize a SparseSet with capacity large enough.
    let mut set = SparseSet::new(u32::MAX as usize);
    // Insert a StateID at the maximum valid value.
    let id: StateID = StateID::from(u32::MAX);
    let _ = set.insert(id); // Insert to ensure it's contained.

    // Call insert with the same id which should return false.
    let result = set.insert(id);
}

