// Answer 0

#[test]
fn test_into_iter_default() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let _iter: IntoIter<u32, String, TestAllocator> = Default::default();
}

#[test]
fn test_into_iter_default_with_global_allocator() {
    let _iter: IntoIter<u32, String> = Default::default();
}

