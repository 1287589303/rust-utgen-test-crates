// Answer 0

#[test]
fn test_data_end_with_min_capacity() {
    let raw_table: RawTable<u8> = RawTable::new_in(Global);
    let _end_ptr: NonNull<u8> = raw_table.data_end();
}

#[test]
fn test_data_end_with_small_capacity() {
    let raw_table: RawTable<u8> = RawTable::with_capacity_in(2, Global);
    let _end_ptr: NonNull<u8> = raw_table.data_end();
}

#[test]
fn test_data_end_with_medium_capacity() {
    let raw_table: RawTable<u8> = RawTable::with_capacity_in(4, Global);
    let _end_ptr: NonNull<u8> = raw_table.data_end();
}

#[test]
fn test_data_end_with_large_capacity() {
    let raw_table: RawTable<u8> = RawTable::with_capacity_in(8, Global);
    let _end_ptr: NonNull<u8> = raw_table.data_end();
}

#[test]
fn test_data_end_power_of_two() {
    let raw_table: RawTable<u8> = RawTable::with_capacity_in(16, Global);
    let _end_ptr: NonNull<u8> = raw_table.data_end();
}

#[test]
fn test_data_end_with_sequential_inserts() {
    let mut raw_table: RawTable<u8> = RawTable::with_capacity_in(4, Global);
    unsafe {
        raw_table.insert(1, 42, |x| x.hash() as u64);
        raw_table.insert(2, 43, |x| x.hash() as u64);
    }
    let _end_ptr: NonNull<u8> = raw_table.data_end();
}

#[test]
fn test_data_end_with_reserve() {
    let mut raw_table: RawTable<u8> = RawTable::with_capacity_in(4, Global);
    raw_table.reserve(2, |x| x.hash() as u64);
    let _end_ptr: NonNull<u8> = raw_table.data_end();
}

