// Answer 0

#[test]
fn test_shrink_to_min_capacity_zero() {
    let mut table = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &_| val.to_owned(); // Simple identity hasher
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.shrink_to(0, hasher);
}

#[test]
fn test_shrink_to_min_capacity_non_zero() {
    let mut table = HashTable::with_capacity_in(100, Global);
    let hasher = |val: &_| val.to_owned(); // Simple identity hasher
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.shrink_to(1, hasher);
}

#[test]
#[should_panic]
fn test_shrink_to_min_capacity_exceeds_current_capacity() {
    let mut table = HashTable::with_capacity_in(5, Global);
    let hasher = |val: &_| val.to_owned(); // Simple identity hasher
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.shrink_to(10, hasher);
}

#[test]
fn test_shrink_to_equal_current_capacity() {
    let mut table = HashTable::with_capacity_in(50, Global);
    let hasher = |val: &_| val.to_owned(); // Simple identity hasher
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.shrink_to(50, hasher);
}

