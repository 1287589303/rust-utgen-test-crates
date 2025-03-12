// Answer 0

#[test]
fn test_fold_with_single_element() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let values = IntoValues {
        inner: IntoIter {
            inner: RawIntoIter::new(vec![(1, "one")].into_iter()),
        },
    };

    let result = values.fold(0, |acc, v| acc + v.len());
}

#[test]
fn test_fold_with_multiple_elements() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let values = IntoValues {
        inner: IntoIter {
            inner: RawIntoIter::new(vec![(1, "one"), (2, "two"), (3, "three")].into_iter()),
        },
    };

    let result = values.fold(0, |acc, v| acc + v.len());
}

#[test]
fn test_fold_with_empty_iterator() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let values = IntoValues {
        inner: IntoIter {
            inner: RawIntoIter::new(vec![].into_iter()),
        },
    };

    let result = values.fold(10, |acc, _v| acc + 1);
}

#[test]
fn test_fold_with_init_value() {
    struct DummyAllocator;

    impl Allocator for DummyAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let values = IntoValues {
        inner: IntoIter {
            inner: RawIntoIter::new(vec![(1, "one"), (2, "two")].into_iter()),
        },
    };

    let result = values.fold(5, |acc, v| acc + v.len());
}

