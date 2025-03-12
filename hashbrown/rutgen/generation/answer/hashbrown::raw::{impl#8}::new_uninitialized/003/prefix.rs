// Answer 0

#[test]
#[should_panic]
fn test_new_uninitialized_with_non_power_of_two_buckets_3() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let buckets = 3; // Not a power of two
    let fallibility = Fallibility::Fallible;

    let _table: Result<RawTable<usize, TestAllocator>, TryReserveError> =
        RawTable::new_uninitialized(allocator, buckets, fallibility);
}

#[test]
#[should_panic]
fn test_new_uninitialized_with_non_power_of_two_buckets_6() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let buckets = 6; // Not a power of two
    let fallibility = Fallibility::Fallible;

    let _table: Result<RawTable<usize, TestAllocator>, TryReserveError> =
        RawTable::new_uninitialized(allocator, buckets, fallibility);
}

#[test]
#[should_panic]
fn test_new_uninitialized_with_non_power_of_two_buckets_10() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let buckets = 10; // Not a power of two
    let fallibility = Fallibility::Fallible;

    let _table: Result<RawTable<usize, TestAllocator>, TryReserveError> =
        RawTable::new_uninitialized(allocator, buckets, fallibility);
}

