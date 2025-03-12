// Answer 0

#[test]
fn test_find_existing_bucket() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(128, 1).unwrap())))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(16, TestAllocator);
    let hash_value: u64 = 5;
    let value_to_insert: u32 = 42;

    unsafe {
        table.insert(hash_value, value_to_insert, |&v| v as u64);
    }

    let found_bucket = table.find(hash_value, |&v| v == value_to_insert);
}

#[test]
fn test_find_multiple_buckets() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(128, 1).unwrap())))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(16, TestAllocator);
    
    let values = vec![10, 20, 30];
    for (i, &value) in values.iter().enumerate() {
        unsafe {
            table.insert(i as u64, value, |&v| v as u64);
        }
    }

    for value in values {
        let found_bucket = table.find(value as u64, |&v| v == value);
    }
}

#[test]
fn test_find_last_bucket() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(128, 1).unwrap())))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(16, TestAllocator);

    let last_index = 15;
    let last_value: u32 = 99;
    
    unsafe {
        table.insert(last_index as u64, last_value, |&v| v as u64);
    }

    let found_bucket = table.find(last_index as u64, |&v| v == last_value);
}

