// Answer 0

#[test]
fn test_new_in_with_global_allocator() {
    use crate::alloc::Global;
    let alloc = Global;
    
    let table: RawTable<i32, Global> = RawTable::new_in(alloc);
}

#[test]
fn test_new_in_with_custom_allocator() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let alloc = CustomAllocator;

    let table: RawTable<i32, CustomAllocator> = RawTable::new_in(alloc);
}

