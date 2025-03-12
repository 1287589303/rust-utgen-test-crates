// Answer 0

#[test]
fn test_into_allocation_non_empty_singleton() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            // Simulate allocation success
            NonNull::new(unsafe { std::alloc::alloc(layout) }).map_or(Err(()), Ok)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let alloc = TestAllocator;
    let mut table = RawTable::<u32, TestAllocator>::new_in(alloc);
    unsafe {
        // Simulate healthy buckets
        table.table.buckets = 4; // Power of two greater than 1
    }

    let _result = table.into_allocation();
}

#[test]
fn test_into_allocation_with_capacity() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            NonNull::new(unsafe { std::alloc::alloc(layout) }).map_or(Err(()), Ok)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let alloc = TestAllocator;
    let mut table = RawTable::<u32, TestAllocator>::with_capacity_in(4, alloc); // Power of two greater than 1

    let _result = table.into_allocation();
}

#[test]
fn test_into_allocation_checked_layout() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            NonNull::new(unsafe { std::alloc::alloc(layout) }).map_or(Err(()), Ok)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let alloc = TestAllocator;
    let mut table = RawTable::<u32, TestAllocator>::new_in(alloc);

    unsafe {
        // Ensure the table has at least two buckets
        table.table.buckets = 4; // Power of two greater than 1
    }

    let _result = table.into_allocation();
}

