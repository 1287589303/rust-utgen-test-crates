// Answer 0

#[test]
fn test_len_empty_iterator() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }
    
    let allocator = TestAllocator;
    let empty_iter = IntoIter { inner: RawIntoIter::new(&allocator, vec![]) };
    let length = empty_iter.len();
}

#[test]
fn test_len_single_element_iterator() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let single_element_iter = IntoIter { inner: RawIntoIter::new(&allocator, vec![1]) };
    let length = single_element_iter.len();
}

#[test]
fn test_len_large_iterator() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let large_iter = IntoIter { inner: RawIntoIter::new(&allocator, (0..1000).collect::<Vec<_>>()) };
    let length = large_iter.len();
}

#[test]
fn test_len_boundary_iterator() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let allocator = TestAllocator;
    let boundary_iter = IntoIter { inner: RawIntoIter::new(&allocator, vec![1, 2, 3, 4, 5]) };
    let length = boundary_iter.len();
}

