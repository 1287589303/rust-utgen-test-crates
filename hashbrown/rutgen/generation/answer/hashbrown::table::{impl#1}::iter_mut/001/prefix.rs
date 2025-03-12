// Answer 0

#[test]
fn test_iter_mut_with_one_element() {
    struct MyAllocator;

    let mut table = HashTable::new_in(MyAllocator);
    let hasher = |val: &i32| (*val as u64).wrapping_add(1);
    table.insert_unique(hasher(&1), 1, hasher);

    let mut iter = table.iter_mut();
    if let Some(val) = iter.next() {
        *val *= 2;
    }
}

#[test]
fn test_iter_mut_with_multiple_elements() {
    struct MyAllocator;

    let mut table = HashTable::new_in(MyAllocator);
    let hasher = |val: &i32| (*val as u64).wrapping_add(1);
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    table.insert_unique(hasher(&3), 3, hasher);

    let mut iter = table.iter_mut();
    while let Some(val) = iter.next() {
        *val *= 2;
    }
}

#[test]
fn test_iter_mut_boundary_conditions() {
    struct MyAllocator;

    let mut table = HashTable::new_in(MyAllocator);
    let hasher = |val: &i32| (*val as u64).wrapping_add(1);
    table.insert_unique(hasher(&1), 1, hasher);
    table.insert_unique(hasher(&2), 2, hasher);
    
    let mut iter = table.iter_mut();
    if let Some(val) = iter.next() {
        *val *= 2;
    }
    // Insert another element to test boundary behavior
    table.insert_unique(hasher(&3), 3, hasher);
    
    while let Some(val) = iter.next() {
        *val *= 2;
    }
}

