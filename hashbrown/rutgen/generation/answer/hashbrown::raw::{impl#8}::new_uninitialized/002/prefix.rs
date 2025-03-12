// Answer 0

#[test]
fn test_new_uninitialized_power_of_two_buckets_infallible() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let allocator = TestAllocator;
    let buckets = 4; // 2^2
    let fallibility = Fallibility::Infallible;

    let result: Result<RawTable<i32, TestAllocator>, TryReserveError> =
        RawTable::new_uninitialized(allocator, buckets, fallibility);
    
    // Usage of result here...
}

#[test]
fn test_new_uninitialized_power_of_two_buckets_fallible() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let allocator = TestAllocator;
    let buckets = 8; // 2^3
    let fallibility = Fallibility::Fallible;

    let result: Result<RawTable<i32, TestAllocator>, TryReserveError> =
        RawTable::new_uninitialized(allocator, buckets, fallibility);
    
    // Usage of result here...
}

#[test]
fn test_new_uninitialized_minimum_power_of_two_buckets() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(layout)))
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            std::alloc::dealloc(ptr.as_ptr(), layout);
        }
    }

    let allocator = TestAllocator;
    let buckets = 1; // 2^0
    let fallibility = Fallibility::Infallible;

    let result: Result<RawTable<i32, TestAllocator>, TryReserveError> =
        RawTable::new_uninitialized(allocator, buckets, fallibility);
    
    // Usage of result here...
}

