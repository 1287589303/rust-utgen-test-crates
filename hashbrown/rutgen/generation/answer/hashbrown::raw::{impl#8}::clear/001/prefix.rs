// Answer 0

#[test]
fn test_clear_empty_table() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 1024]))))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // Do nothing for testing
        }
    }

    let allocator = TestAllocator;
    let mut table: RawTable<u8, TestAllocator> = RawTable::new_in(allocator);
    table.clear();
}

#[test]
fn test_clear_no_elements() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new([0u8; 1024]))))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // Do nothing for testing
        }
    }

    let allocator = TestAllocator;
    let mut table: RawTable<u8, TestAllocator> = RawTable::new_in(allocator);
    table.clear();
}

