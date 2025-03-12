// Answer 0

#[test]
fn test_new_uninitialized_fallible_err() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            // Returning a simulated allocation failure
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No-op for deallocation in this test
        }
    }

    let alloc = TestAllocator;
    let fallibility = Fallibility::Fallible;

    // Test with buckets = 1, which is a power of two
    let result = RawTable::<u8, TestAllocator>::new_uninitialized(alloc, 1, fallibility);
}

#[test]
fn test_new_uninitialized_infallible_err() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            // Returning a simulated allocation failure
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No-op for deallocation in this test
        }
    }

    let alloc = TestAllocator;
    let fallibility = Fallibility::Infallible;

    // Test with buckets = 2, which is a power of two
    let result = RawTable::<u8, TestAllocator>::new_uninitialized(alloc, 2, fallibility);
}

#[test]
fn test_new_uninitialized_large_power_of_two_err() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            // Returning a simulated allocation failure
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No-op for deallocation in this test
        }
    }

    let alloc = TestAllocator;
    let fallibility = Fallibility::Fallible;

    // Test with buckets = 16, which is a power of two
    let result = RawTable::<u8, TestAllocator>::new_uninitialized(alloc, 16, fallibility);
}

#[test]
fn test_new_uninitialized_max_power_of_two_err() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            // Returning a simulated allocation failure
            Err(())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            // No-op for deallocation in this test
        }
    }

    let alloc = TestAllocator;
    let fallibility = Fallibility::Infallible;

    // Test with buckets = 1073741824 (2^30), which is a power of two
    let result = RawTable::<u8, TestAllocator>::new_uninitialized(alloc, 1073741824, fallibility);
}

