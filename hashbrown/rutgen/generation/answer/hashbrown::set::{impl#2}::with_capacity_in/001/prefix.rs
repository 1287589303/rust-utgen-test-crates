// Answer 0

#[test]
fn test_with_capacity_zero() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = DummyAllocator;
    let set: HashSet<i32, DefaultHashBuilder, DummyAllocator> = HashSet::with_capacity_in(0, alloc);
}

#[test]
fn test_with_capacity_one() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = DummyAllocator;
    let set: HashSet<i32, DefaultHashBuilder, DummyAllocator> = HashSet::with_capacity_in(1, alloc);
}

#[test]
fn test_with_capacity_ten() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = DummyAllocator;
    let set: HashSet<i32, DefaultHashBuilder, DummyAllocator> = HashSet::with_capacity_in(10, alloc);
}

#[test]
fn test_with_capacity_edge_cases() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = DummyAllocator;

    // Below the lower boundary
    let set_below = HashSet::with_capacity_in(usize::MAX, alloc);
    
    // At the upper boundary
    let set_at_upper = HashSet::with_capacity_in(10, alloc);
}

