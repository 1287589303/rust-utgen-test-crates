// Answer 0

#[test]
fn test_next_empty_iterator() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let iter: RawIntoIter<(i32, &str), TestAllocator> = RawIntoIter {
        iter: RawIter::empty(),
        allocation: None,
        marker: PhantomData,
    };
    let mut into_iter = IntoIter { inner: iter };
    let result = into_iter.next();
}

#[test]
fn test_next_single_item_iterator() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let item = (1, "test");
    let iter: RawIntoIter<(i32, &str), TestAllocator> = RawIntoIter {
        iter: RawIter::from_single(&item),
        allocation: None,
        marker: PhantomData,
    };
    let mut into_iter = IntoIter { inner: iter };
    let result = into_iter.next();
}

#[test]
fn test_next_multi_item_iterator() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let items = vec![(1, "first"), (2, "second"), (3, "third")];
    let iter: RawIntoIter<(i32, &str), TestAllocator> = RawIntoIter {
        iter: RawIter::from_vec(items),
        allocation: None,
        marker: PhantomData,
    };
    let mut into_iter = IntoIter { inner: iter };
    
    for _ in 0..3 {
        let result = into_iter.next();
    }
}

#[test]
fn test_next_exhausted_iterator() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let items = vec![(1, "first"), (2, "second")];
    let iter: RawIntoIter<(i32, &str), TestAllocator> = RawIntoIter {
        iter: RawIter::from_vec(items),
        allocation: None,
        marker: PhantomData,
    };
    let mut into_iter = IntoIter { inner: iter };
    
    for _ in 0..2 {
        let result = into_iter.next();
    }
    
    let result = into_iter.next();
}

