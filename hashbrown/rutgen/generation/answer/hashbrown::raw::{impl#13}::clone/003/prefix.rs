// Answer 0

#[test]
fn test_clone_non_empty_singleton() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let alloc = TestAllocator;
    let buckets = 4; // power of two greater than 1
    let mut table = unsafe {
        RawTable::new_uninitialized(alloc.clone(), buckets, Fallibility::Infallible).unwrap()
    };

    // Simulating non-empty state
    table.items = 1;

    let raw_table = RawTable {
        table,
        alloc,
        marker: PhantomData,
    };

    let _cloned_table = raw_table.clone();
}

#[test]
fn test_clone_with_non_zero_items() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let alloc = TestAllocator;
    let buckets = 8; // power of two greater than 1
    let mut table = unsafe {
        RawTable::new_uninitialized(alloc.clone(), buckets, Fallibility::Infallible).unwrap()
    };

    // Simulating non-empty state
    table.items = 5;

    let raw_table = RawTable {
        table,
        alloc,
        marker: PhantomData,
    };

    let _cloned_table = raw_table.clone();
}

