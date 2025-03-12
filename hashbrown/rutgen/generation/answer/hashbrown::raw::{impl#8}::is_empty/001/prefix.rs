// Answer 0

#[test]
fn test_raw_table_is_empty_when_empty() {
    let table: RawTable<u32> = RawTable::new_in(Global);
    assert!(table.is_empty());
}

#[test]
fn test_raw_table_is_empty_when_non_empty() {
    let mut table: RawTable<u32> = RawTable::with_capacity_in(1, Global);
    unsafe {
        table.insert(1, 42, |x| *x);
    }
    assert!(!table.is_empty());
}

