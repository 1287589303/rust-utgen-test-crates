// Answer 0

#[test]
fn test_into_iter_non_empty() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    table.insert_unique(1, "value", |s: &str| s.len() as u64);
    let mut iter = table.iter_mut();
    let first = iter.next();
}

#[test]
fn test_into_iter_with_capacity() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table = HashTable::with_capacity_in(10, TestAllocator);
    table.insert_unique(2, "another_value", |s: &str| s.len() as u64);
    let mut iter = table.iter_mut();
    let first = iter.next();
}

#[test]
fn test_into_iter_multiple_entries() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    table.insert_unique(3, "entry_one", |s: &str| s.len() as u64);
    table.insert_unique(4, "entry_two", |s: &str| s.len() as u64);
    let mut iter = table.iter_mut();
    let first = iter.next();
    let second = iter.next();
}

#[test]
fn test_into_iter_after_clear() {
    struct TestAllocator;
    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    table.insert_unique(5, "sample", |s: &str| s.len() as u64);
    table.clear();
    let mut iter = table.iter_mut();
    let first_after_clear = iter.next();
}

