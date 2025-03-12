// Answer 0

#[test]
fn test_bucket_with_index_zero() {
    unsafe {
        let alloc = &Global;  // Use a global allocator
        let table_layout = TableLayout::default();  // Assume default layout for simplicity
        let capacity = 1;  // Must be power of two
        let mut raw_table = RawTableInner::with_capacity(alloc, table_layout, capacity);
        let bucket = raw_table.bucket::<i32>(0);  // Index should be valid
        let pointer = bucket.as_ptr();  // Call the method to get pointer
    }
}

#[test]
fn test_bucket_with_index_one() {
    unsafe {
        let alloc = &Global;
        let table_layout = TableLayout::default();
        let capacity = 2;  // Must be power of two
        let mut raw_table = RawTableInner::with_capacity(alloc, table_layout, capacity);
        let bucket = raw_table.bucket::<i32>(1);  // Index should be valid
        let pointer = bucket.as_ptr();  // Call the method to get pointer
    }
}

#[test]
fn test_bucket_with_index_at_capacity_minus_one() {
    unsafe {
        let alloc = &Global;
        let table_layout = TableLayout::default();
        let capacity = 4;  // Must be power of two
        let mut raw_table = RawTableInner::with_capacity(alloc, table_layout, capacity);
        let bucket = raw_table.bucket::<i32>(3);  // Index should be valid
        let pointer = bucket.as_ptr();  // Call the method to get pointer
    }
}

