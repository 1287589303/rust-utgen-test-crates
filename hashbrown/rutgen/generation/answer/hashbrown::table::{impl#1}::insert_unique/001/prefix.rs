// Answer 0

#[test]
fn insert_unique_with_small_values() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<i32, TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &i32| *val as u64;
    let hash = hasher(&1);
    table.insert_unique(hash, 1, hasher);
}

#[test]
fn insert_unique_with_large_values() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<i64, TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &i64| *val as u64;
    let hash = hasher(&i64::MAX);
    table.insert_unique(hash, i64::MAX, hasher);
}

#[test]
fn insert_unique_with_negative_values() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<i32, TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &i32| *val as u64;
    let hash = hasher(&-1);
    table.insert_unique(hash, -1, hasher);
}

#[test]
fn insert_unique_with_zero_value() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<u32, TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &u32| *val as u64;
    let hash = hasher(&0);
    table.insert_unique(hash, 0, hasher);
}

#[test]
fn insert_unique_with_boundary_hash_values() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table: HashTable<u64, TestAllocator> = HashTable::new_in(TestAllocator);
    let hasher = |val: &u64| *val;
    let min_hash = 1u64;
    let max_hash = u64::MAX;
    table.insert_unique(min_hash, 1, hasher);
    table.insert_unique(max_hash, 2, hasher);
}

