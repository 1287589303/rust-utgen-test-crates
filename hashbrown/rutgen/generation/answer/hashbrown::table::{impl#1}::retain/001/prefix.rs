// Answer 0

#[test]
fn test_retain_even_numbers_only() {
    struct TestAllocator;

    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &_| 0; // Dummy hasher that produces the same hash for all values
    for x in 1..=6 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x % 2 == 0);
}

#[test]
fn test_retain_no_elements() {
    struct TestAllocator;

    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &_| 0; // Dummy hasher that produces the same hash for all values
    for x in 1..=6 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x > 6); // All items will be removed
}

#[test]
fn test_retain_only_one_element() {
    struct TestAllocator;

    impl Allocator for TestAllocator {}

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &_| 0; // Dummy hasher that produces the same hash for all values
    for x in 1..=6 {
        table.insert_unique(hasher(&x), x, hasher);
    }
    table.retain(|&mut x| x == 4); // Only item 4 should remain
}

