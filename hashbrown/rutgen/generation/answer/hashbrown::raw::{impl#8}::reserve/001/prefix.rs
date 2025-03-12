// Answer 0

#[test]
fn test_reserve_exceeds_growth_left_and_alloc_error() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(()) // Simulate allocation failure
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let mut table = RawTable::with_capacity_in(1, alloc); // Start with minimal capacity

    // Force growth_left to be something smaller
    table.table.growth_left = 1; 
    let additional = 2; // Set additional greater than growth_left

    unsafe {
        table.reserve(additional, |value| 0); // Providing a dummy hasher
    }
}

#[test]
fn test_reserve_exceeds_capacity_overflow() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(()) // Simulate allocation failure
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let mut table = RawTable::with_capacity_in(1, alloc); // Start with minimal capacity

    // Force growth_left to be something smaller
    table.table.growth_left = 1; 
    let additional = isize::MAX as usize + 1; // Set additional beyond maximum capacity

    unsafe {
        table.reserve(additional, |value| 0); // Providing a dummy hasher
    }
}

