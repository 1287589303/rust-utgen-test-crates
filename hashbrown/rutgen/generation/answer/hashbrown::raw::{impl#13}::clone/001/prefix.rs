// Answer 0

#[test]
fn test_clone_empty_singleton() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0u8)) as *mut u8))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
            drop(Box::from_raw(ptr.as_ptr() as *mut u8));
        }
    }

    let allocator = TestAllocator;
    let table: RawTable<u8, TestAllocator> = RawTable::new_in(allocator);

    let cloned_table = table.clone();
}

#[test]
fn test_clone_empty_singleton_with_capacity_zero() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(Box::into_raw(Box::new(0u8)) as *mut u8))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
            drop(Box::from_raw(ptr.as_ptr() as *mut u8));
        }
    }

    let allocator = TestAllocator;
    let table: RawTable<u8, TestAllocator> = RawTable::with_capacity_in(0, allocator);

    let cloned_table = table.clone();
}

