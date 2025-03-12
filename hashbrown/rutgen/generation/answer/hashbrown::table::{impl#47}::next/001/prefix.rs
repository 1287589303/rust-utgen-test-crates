// Answer 0

#[test]
fn test_extract_if_returns_none_when_empty() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::with_capacity(0);
    let extractor = ExtractIf {
        f: |_: &mut i32| false,
        inner: RawExtractIf {
            iter: RawIter::new(&mut table),
            table: &mut table,
        },
    };

    let mut iter = extractor;
    let result = iter.next();
}

#[test]
fn test_extract_if_filters_correctly() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::with_capacity(3);
    table.insert(1);
    table.insert(2);
    table.insert(3);

    let extractor = ExtractIf {
        f: |val: &mut i32| *val % 2 == 0,
        inner: RawExtractIf {
            iter: RawIter::new(&mut table),
            table: &mut table,
        },
    };

    let mut iter = extractor;

    let result1 = iter.next();
    let result2 = iter.next();
    let result3 = iter.next();
}

#[test]
fn test_extract_if_mutably_references_values() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::with_capacity(3);
    table.insert(10);
    table.insert(20);
    table.insert(30);

    let mut total = 0;

    let mut extractor = ExtractIf {
        f: |val: &mut i32| {
            total += *val;
            *val > 15
        },
        inner: RawExtractIf {
            iter: RawIter::new(&mut table),
            table: &mut table,
        },
    };

    let _ = extractor.next();
    let _ = extractor.next();
}

#[test]
fn test_extract_if_empty_table_with_condition() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Err(()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut table = RawTable::with_capacity(0);

    let extractor = ExtractIf {
        f: |_: &mut i32| true,
        inner: RawExtractIf {
            iter: RawIter::new(&mut table),
            table: &mut table,
        },
    };

    let mut iter = extractor;
    let result = iter.next();
}

