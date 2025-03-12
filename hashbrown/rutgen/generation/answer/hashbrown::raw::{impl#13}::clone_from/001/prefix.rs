// Answer 0

#[test]
fn test_clone_from_empty_singleton() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let alloc = TestAllocator;
    
    let source: RawTable<u32, TestAllocator> = RawTable::new_in(alloc);
    let mut destination: RawTable<u32, TestAllocator> = RawTable::new_in(alloc);
    
    destination.clone_from(&source);
}

#[test]
fn test_clone_from_with_zero_items() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;

    let source: RawTable<u32, TestAllocator> = RawTable::new_in(alloc);
    let mut destination: RawTable<u32, TestAllocator> = RawTable::new_in(alloc);
    
    destination.clone_from(&source);
}

#[test]
fn test_clone_from_with_power_of_two_buckets() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    
    let source: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(2, alloc);
    let mut destination: RawTable<u32, TestAllocator> = RawTable::new_in(alloc);
    
    destination.clone_from(&source);
}

#[test]
fn test_clone_from_with_non_empty_source() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;

    let source: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(2, alloc);
    let mut destination: RawTable<u32, TestAllocator> = RawTable::new_in(alloc);

    // This tree assumes we have added items to the source before cloning
    // Adding a number of items to source for the sake of testing
    destination.insert(1, 42, |x| *x);
    
    destination.clone_from(&source);
}

