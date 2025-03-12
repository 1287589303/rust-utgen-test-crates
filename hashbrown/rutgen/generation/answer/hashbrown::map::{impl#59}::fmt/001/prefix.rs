// Answer 0

#[test]
fn test_fmt_empty_iterable() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let empty_vec: Vec<(String, String)> = Vec::new();
    let iter = IntoIter {
        inner: RawIntoIter {
            iter: RawIter::from(empty_vec.into_iter()),
            allocation: None,
            marker: PhantomData,
        },
    };

    let _ = fmt::Debug::fmt(&iter, &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_single_element_iterable() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let single_element_vec = vec![("key1".to_string(), "value1".to_string())];
    let iter = IntoIter {
        inner: RawIntoIter {
            iter: RawIter::from(single_element_vec.into_iter()),
            allocation: None,
            marker: PhantomData,
        },
    };

    let _ = fmt::Debug::fmt(&iter, &mut fmt::Formatter::new());
}

#[test]
fn test_fmt_large_iterable() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let large_vec: Vec<(i32, i32)> = (0..1000).map(|x| (x, x * 2)).collect();
    let iter = IntoIter {
        inner: RawIntoIter {
            iter: RawIter::from(large_vec.into_iter()),
            allocation: None,
            marker: PhantomData,
        },
    };

    let _ = fmt::Debug::fmt(&iter, &mut fmt::Formatter::new());
}

