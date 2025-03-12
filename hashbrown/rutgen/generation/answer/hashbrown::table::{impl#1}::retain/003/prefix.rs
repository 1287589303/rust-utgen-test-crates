// Answer 0

#[test]
fn test_retain_all_elements_removed() {
    struct TestAllocator;

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &_| val;

    for x in 1..=5 {
        table.insert_unique(hasher(&x), x, hasher);
    }

    table.retain(|&mut x| x % 2 == 1); // Predicate returns false for all elements
}

#[test]
fn test_retain_single_element_remaining() {
    struct TestAllocator;

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &_| val;

    for x in 1..=5 {
        table.insert_unique(hasher(&x), x, hasher);
    }

    table.retain(|&mut x| x > 4); // Predicate will return false for 1, 2, 3, 4
}

#[test]
fn test_retain_empty_after_removal() {
    struct TestAllocator;

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &_| val;

    table.insert_unique(hasher(&1), 1, hasher);

    table.retain(|&mut x| false); // Predicate returns false for all elements
}

#[test]
fn test_retain_with_multiple_elements_all_removed() {
    struct TestAllocator;

    let mut table = HashTable::new_in(TestAllocator);
    let hasher = |val: &_| val;

    for x in 10..=15 {
        table.insert_unique(hasher(&x), x, hasher);
    }

    table.retain(|&mut x| x < 10); // Predicate returns false for all elements
}

