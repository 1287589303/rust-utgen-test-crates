// Answer 0

#[test]
fn test_drain_empty() {
    let table: RawTable<u32> = RawTable::with_capacity_in(0, Global);
    let drain = unsafe { table.drain() };
}

#[test]
fn test_drain_single_item() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(1, Global);
    unsafe {
        table.insert(1, 42, |x| *x);
    }
    let drain = unsafe { table.drain() };
}

#[test]
fn test_drain_multiple_items() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(4, Global);
    unsafe {
        table.insert(1, 42, |x| *x);
        table.insert(2, 43, |x| *x);
        table.insert(3, 44, |x| *x);
    }
    let drain = unsafe { table.drain() };
}

#[test]
fn test_drain_with_reallocations() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(2, Global);
    unsafe {
        table.insert(1, 42, |x| *x);
        table.insert(2, 43, |x| *x);
        table.reserve(2, |x| *x);
    }
    let drain = unsafe { table.drain() };
}

#[test]
fn test_drain_after_clear() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(3, Global);
    unsafe {
        table.insert(1, 42, |x| *x);
        table.clear();
    }
    let drain = unsafe { table.drain() };
}

