// Answer 0

#[test]
fn test_insert_success() {
    let mut set = SparseSet::new(10);
    set.insert(0);
    set.insert(1);
    set.insert(2);
    let id = 3; // New ID to insert
    set.insert(id); // This should succeed as it is not in the set
}

#[test]
#[should_panic]
fn test_insert_capacity_exceeded() {
    let mut set = SparseSet::new(2);
    set.insert(0); // Insert first ID
    set.insert(1); // Insert second ID
    let id = 2; // ID to insert, should exceed capacity now
    set.insert(id); // This should panic as capacity will be exceeded
}

#[test]
#[should_panic]
fn test_insert_with_length_equals_capacity() {
    let mut set = SparseSet::new(3);
    set.insert(0);
    set.insert(1);
    set.insert(2);
    let id = 3; // ID to insert, should create an edge case since length equals capacity
    set.insert(id); // This should panic as it exceeds capacity
}

