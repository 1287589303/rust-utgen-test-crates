// Answer 0

#[test]
fn test_len_non_empty_drain_with_default_allocator() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let map: HashMap<i32, i32, DefaultHashBuilder> = HashMap::with_capacity(10);
    let mut drain = Drain { iter: map.drain() };

    // Populate the drain with elements
    for i in 0..10 {
        drain.insert(i, i);
    }

    let length = drain.len();
}

#[test]
fn test_len_empty_drain_with_default_allocator() {
    let map: HashMap<i32, i32, DefaultHashBuilder> = HashMap::with_capacity(0);
    let drain = Drain { iter: map.drain() };

    let length = drain.len();
}

#[test]
fn test_len_non_empty_drain_with_custom_allocator() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let map: HashMap<i32, i32, TestAllocator> = HashMap::with_capacity(10);
    let mut drain = Drain { iter: map.drain() };

    // Populate the drain with elements
    for i in 0..10 {
        drain.insert(i, i);
    }

    let length = drain.len();
}

#[test]
fn test_len_boundary_case_drain() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let map: HashMap<i32, i32, TestAllocator> = HashMap::with_capacity(std::usize::MAX);
    let drain = Drain { iter: map.drain() };

    let length = drain.len();
}

