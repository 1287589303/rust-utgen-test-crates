// Answer 0

#[test]
fn test_next_with_non_empty_hashmap() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<(u32, ()), TestAllocator> = RawTable::with_capacity(10);
    table.insert((1, ()));
    table.insert((2, ()));

    let extractor = ExtractIf {
        f: |&k| k % 2 == 0,
        inner: RawExtractIf {
            iter: RawIter::new(),
            table: &mut table,
        },
    };

    let mut iter = extractor;
    let _ = iter.next();
}

#[test]
fn test_next_with_empty_hashmap() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<(u32, ()), TestAllocator> = RawTable::with_capacity(0);

    let extractor = ExtractIf {
        f: |&_| true,
        inner: RawExtractIf {
            iter: RawIter::new(),
            table: &mut table,
        },
    };

    let mut iter = extractor;
    let _ = iter.next();
}

#[test]
fn test_next_with_specific_condition() {
    struct TestAllocator;
    
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<(u32, ()), TestAllocator> = RawTable::with_capacity(5);
    table.insert((3, ()));
    table.insert((4, ()));

    let extractor = ExtractIf {
        f: |&k| k == 4,
        inner: RawExtractIf {
            iter: RawIter::new(),
            table: &mut table,
        },
    };

    let mut iter = extractor;
    let _ = iter.next();
}

#[test]
fn test_next_with_allocator_limit() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table: RawTable<(u64, ()), TestAllocator> = RawTable::with_capacity(10);
    for i in 0..10 {
        table.insert((i, ()));
    }

    let extractor = ExtractIf {
        f: |&k| k < 10,
        inner: RawExtractIf {
            iter: RawIter::new(),
            table: &mut table,
        },
    };

    let mut iter = extractor;
    let _ = iter.next();
}

