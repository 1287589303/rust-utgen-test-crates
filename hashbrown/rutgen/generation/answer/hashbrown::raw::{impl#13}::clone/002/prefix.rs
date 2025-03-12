// Answer 0

#[test]
#[should_panic]
fn test_clone_empty_singleton_false_allocation_failure() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = MockAllocator;
    let mut table = RawTable::with_capacity_in(1, allocator);

    // Simulate adding an item to ensure is_empty_singleton is false
    unsafe {
        let item = 42; // Example value, assuming T is usize
        let hash = 123; // Example hash
        table.insert(hash, item, |x| *x); // Inserting an item
    }

    // Attempting to clone, expecting it to panic due to allocation failure
    let _ = table.clone();
}

#[test]
#[should_panic]
fn test_clone_allocation_failure_non_empty() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }

        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = MockAllocator;
    let mut table = RawTable::with_capacity_in(2, allocator);

    // Simulate adding items to ensure capacity is met
    unsafe {
        table.insert(1, 10, |x| *x);
        table.insert(2, 20, |x| *x);
    }

    // Attempting to clone, expecting it to panic
    let _ = table.clone();
}

