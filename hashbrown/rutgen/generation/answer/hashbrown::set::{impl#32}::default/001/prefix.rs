// Answer 0

#[test]
fn test_default_into_iter_with_default_types() {
    let iter: IntoIter<u32, Global> = IntoIter::default();
    let _ = iter;
}

#[test]
fn test_default_into_iter_with_custom_type() {
    struct CustomAllocator;
    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            unimplemented!()
        }
    }
    let iter: IntoIter<String, CustomAllocator> = IntoIter::default();
    let _ = iter;
}

#[test]
fn test_default_into_iter_with_string() {
    let iter: IntoIter<String, Global> = IntoIter::default();
    let _ = iter;
}

#[test]
fn test_default_into_iter_with_i32() {
    let iter: IntoIter<i32, Global> = IntoIter::default();
    let _ = iter;
}

