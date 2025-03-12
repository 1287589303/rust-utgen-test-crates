// Answer 0

#[test]
fn test_into_keys_next_non_empty() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let keys = vec![1, 2, 3];
    let values = vec!["a", "b", "c"];
    let allocator = TestAllocator;

    let into_keys: IntoKeys<_, _, _> = IntoKeys {
        inner: IntoIter {
            inner: RawIntoIter::from_iter(keys.into_iter().zip(values.into_iter()), allocator),
        },
    };

    let mut iter = into_keys;
    let _ = iter.next();
}

#[test]
fn test_into_keys_next_empty() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let keys: Vec<i32> = vec![];
    let values: Vec<&str> = vec![];
    let allocator = TestAllocator;

    let into_keys: IntoKeys<_, _, _> = IntoKeys {
        inner: IntoIter {
            inner: RawIntoIter::from_iter(keys.into_iter().zip(values.into_iter()), allocator),
        },
    };

    let mut iter = into_keys;
    let _ = iter.next();
}

