// Answer 0

#[test]
fn test_default_into_iter_with_default_allocator() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let iter: IntoIter<i32, TestAllocator> = Default::default();
}

#[test]
fn test_default_into_iter_with_default_type() {
    let iter: IntoIter<()> = Default::default();
}

