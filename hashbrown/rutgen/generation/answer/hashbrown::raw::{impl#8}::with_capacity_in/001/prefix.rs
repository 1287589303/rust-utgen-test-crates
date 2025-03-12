// Answer 0

#[test]
fn test_with_capacity_in_zero_capacity() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(0, alloc);
}

#[test]
fn test_with_capacity_in_small_capacity() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(1, alloc);
}

#[test]
fn test_with_capacity_in_large_capacity() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(usize::MAX, alloc);
}

#[test]
fn test_with_capacity_in_mid_capacity() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(1024, alloc);
}

