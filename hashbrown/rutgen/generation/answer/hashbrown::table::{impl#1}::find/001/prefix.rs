// Answer 0

#[test]
fn test_find_with_present_element() {
    struct TestAllocator;

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &u64| *val;

    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);

    table.find(hasher(&2), |&val| val == 2);
}

#[test]
fn test_find_with_absent_element() {
    struct TestAllocator;

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &u64| *val;

    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);

    table.find(hasher(&3), |&val| val == 3);
}

#[test]
fn test_find_in_empty_table() {
    struct TestAllocator;

    let table: HashTable<u64, TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &u64| *val;

    table.find(hasher(&1), |&val| val == 1);
}

#[test]
fn test_find_with_custom_equality() {
    struct TestAllocator;

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &u64| *val;

    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);

    table.find(hasher(&1), |&val| val % 2 == 1);
}

#[test]
fn test_find_with_edge_case_hash() {
    struct TestAllocator;

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &u64| *val;

    table.insert_unique(0, 0, hasher);
    
    table.find(0, |&val| val == 0);
}

#[test]
fn test_find_with_large_hash() {
    struct TestAllocator;

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &u64| *val;

    table.insert_unique(u64::MAX, 42, hasher);
    
    table.find(u64::MAX, |&val| val == 42);
}

