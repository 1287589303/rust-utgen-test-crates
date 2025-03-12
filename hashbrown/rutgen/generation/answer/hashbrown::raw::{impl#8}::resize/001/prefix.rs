// Answer 0

#[test]
fn test_resize_inflation() {
    let mut table: RawTable<u32> = RawTable::new_in(Global);
    // Initialize table with some items
    for i in 0..10 {
        unsafe {
            table.insert(i as u64, i, |x| *x);
        }
    }
    let old_capacity = table.capacity();
    let new_capacity = old_capacity + 10; // Higher than current items, valid
    unsafe {
        table.resize(new_capacity, |x| *x, Fallibility::Infallible).unwrap();
    }
}

#[test]
#[should_panic]
fn test_resize_capacity_zero() {
    let mut table: RawTable<u32> = RawTable::new_in(Global);
    // Initialize table with some items
    for i in 0..5 {
        unsafe {
            table.insert(i as u64, i, |x| *x);
        }
    }
    let new_capacity = 0; // Invalid, as items are non-zero
    unsafe {
        table.resize(new_capacity, |x| *x, Fallibility::Infallible).unwrap();
    }
}

#[test]
#[should_panic]
fn test_resize_to_capacity_too_small() {
    let mut table: RawTable<u32> = RawTable::new_in(Global);
    // Initialize table with items such that current items > capacity buckets allowed
    for i in 0..20 {
        unsafe {
            table.insert(i as u64, i, |x| *x);
        }
    }
    let new_capacity = 1; // Invalid, current items exceed capacity buckets
    unsafe {
        table.resize(new_capacity, |x| *x, Fallibility::Infallible).unwrap();
    }
}

#[test]
fn test_resize_no_items() {
    let mut table: RawTable<u32> = RawTable::new_in(Global);
    let new_capacity = 16; // Valid as table is empty
    unsafe {
        table.resize(new_capacity, |x| *x, Fallibility::Infallible).unwrap();
    }
}

#[test]
fn test_resize_min_capacity() {
    let mut table: RawTable<u32> = RawTable::new_in(Global);
    // Initialize table with some items
    for i in 0..8 {
        unsafe {
            table.insert(i as u64, i, |x| *x);
        }
    }
    let new_capacity = 8; // Valid case where new capacity equals current items
    unsafe {
        table.resize(new_capacity, |x| *x, Fallibility::Infallible).unwrap();
    }
}

#[test]
fn test_resize_large_capacity() {
    let mut table: RawTable<u32> = RawTable::new_in(Global);
    // Fill the table with items
    for i in 0..10 {
        unsafe {
            table.insert(i as u64, i, |x| *x);
        }
    }
    let new_capacity = isize::MAX as usize; // Maximum value, still valid
    unsafe {
        table.resize(new_capacity, |x| *x, Fallibility::Infallible).unwrap();
    }
}

