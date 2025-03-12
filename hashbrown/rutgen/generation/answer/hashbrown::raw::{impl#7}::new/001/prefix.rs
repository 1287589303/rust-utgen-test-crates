// Answer 0

#[test]
fn test_raw_table_new() {
    let table: RawTable<u8> = RawTable::new();
}

#[test]
fn test_raw_table_new_with_different_type() {
    let table: RawTable<f64> = RawTable::new();
}

