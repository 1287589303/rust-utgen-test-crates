// Answer 0

#[test]
fn test_clear_no_drop_empty_table() {
    let table: RawTable<u32> = RawTable::new_in(Global);
    unsafe {
        table.clear_no_drop();
    }
}

#[test]
fn test_clear_no_drop_single_item() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(1, Global);
    unsafe {
        table.insert(1, 42, |x| *x); // Simulating an insertion
        table.clear_no_drop();
    }
}

#[test]
fn test_clear_no_drop_multiple_items() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(4, Global);
    unsafe {
        table.insert(1, 42, |x| *x);
        table.insert(2, 43, |x| *x);
        table.insert(3, 44, |x| *x);
        table.clear_no_drop();
    }
}

#[test]
fn test_clear_no_drop_power_of_two_items() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(8, Global);
    unsafe {
        table.insert(1, 42, |x| *x);
        table.insert(2, 43, |x| *x);
        table.insert(3, 44, |x| *x);
        table.insert(4, 45, |x| *x);
        table.clear_no_drop();
    }
}

#[test]
fn test_clear_no_drop_large_table() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(16, Global);
    unsafe {
        table.insert(1, 10, |x| *x);
        table.insert(2, 20, |x| *x);
        table.insert(3, 30, |x| *x);
        table.insert(4, 40, |x| *x);
        table.insert(5, 50, |x| *x);
        table.insert(6, 60, |x| *x);
        table.clear_no_drop();
    }
}

