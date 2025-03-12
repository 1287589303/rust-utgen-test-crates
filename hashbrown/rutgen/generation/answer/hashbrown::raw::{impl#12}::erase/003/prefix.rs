// Answer 0

#[test]
unsafe fn test_erase_bucket_full_true() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 16;
    let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, capacity);

    let index = 0; // Assuming index 0 is full
    raw_table.items = 1; // Set items greater than 0

    // Pre-fill the control byte to be full to satisfy is_bucket_full
    raw_table.set_ctrl(index, Tag(0xff)); // Simulate full

    raw_table.erase(index); // Call function being tested
}

#[test]
unsafe fn test_erase_bucket_full_index_boundary() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 16;
    let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, capacity);

    let index = raw_table.bucket_mask; // Highest index
    raw_table.items = 1; // Set items greater than 0

    // Pre-fill the control byte to be full to satisfy is_bucket_full
    raw_table.set_ctrl(index, Tag(0xff)); // Simulate full

    raw_table.erase(index); // Call function being tested
}

#[test]
unsafe fn test_erase_bucket_full_multiple() {
    let allocator = Global;
    let table_layout = TableLayout::default();
    let capacity = 32;
    let mut raw_table = RawTableInner::with_capacity(&allocator, table_layout, capacity);

    let index = 15; // Assuming index 15 is full
    raw_table.items = 5; // Set items greater than 0

    // Pre-fill the control byte to be full to satisfy is_bucket_full
    raw_table.set_ctrl(index, Tag(0xff)); // Simulate full

    raw_table.erase(index); // Call function being tested
}

