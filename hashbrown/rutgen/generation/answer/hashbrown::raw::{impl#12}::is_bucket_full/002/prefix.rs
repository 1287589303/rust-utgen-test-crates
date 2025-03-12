// Answer 0

#[test]
#[should_panic]
unsafe fn test_is_bucket_full_index_equals_buckets() {
    let alloc = Global; // Using the global allocator
    let table_layout = TableLayout::default(); // Default or any valid layout
    let capacity = 4; // Example capacity that translates to buckets
    let raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    let index = raw_table.buckets(); // Set index to buckets, which is out of bounds
    raw_table.is_bucket_full(index);
}

#[test]
#[should_panic]
unsafe fn test_is_bucket_full_index_exceeds_buckets() {
    let alloc = Global; // Using the global allocator
    let table_layout = TableLayout::default(); // Default or any valid layout
    let capacity = 4; // Example capacity that translates to buckets
    let raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    let index = raw_table.buckets() + 1; // Set index to exceed buckets
    raw_table.is_bucket_full(index);
}

#[test]
#[should_panic]
unsafe fn test_is_bucket_full_index_negative() {
    let alloc = Global; // Using the global allocator
    let table_layout = TableLayout::default(); // Default or any valid layout
    let capacity = 4; // Example capacity that translates to buckets
    let raw_table = RawTableInner::with_capacity(&alloc, table_layout, capacity);
    let index = usize::MAX; // Use a negative index represented by a large value
    raw_table.is_bucket_full(index);
}

