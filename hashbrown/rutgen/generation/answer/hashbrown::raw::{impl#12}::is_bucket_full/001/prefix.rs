// Answer 0

#[test]
fn test_is_bucket_full_valid_index_zero() {
    let alloc = Global; // Using default global allocator
    let table_layout = TableLayout::default(); // Assuming a default layout
    let capacity = 1; // Minimum capacity to have at least one bucket
    let table_inner = unsafe { RawTableInner::with_capacity(&alloc, table_layout, capacity) };
    unsafe {
        let result = table_inner.is_bucket_full(0);
    }
}

#[test]
fn test_is_bucket_full_valid_index_mid() {
    let alloc = Global; 
    let table_layout = TableLayout::default();
    let capacity = 4; // Enough capacity for testing in the middle index
    let table_inner = unsafe { RawTableInner::with_capacity(&alloc, table_layout, capacity) };
    unsafe {
        let result = table_inner.is_bucket_full(2);
    }
}

#[test]
fn test_is_bucket_full_valid_index_max() {
    let alloc = Global; 
    let table_layout = TableLayout::default();
    let capacity = 8; // Ensure enough buckets
    let table_inner = unsafe { RawTableInner::with_capacity(&alloc, table_layout, capacity) };
    unsafe {
        let result = table_inner.is_bucket_full(7);
    }
}

