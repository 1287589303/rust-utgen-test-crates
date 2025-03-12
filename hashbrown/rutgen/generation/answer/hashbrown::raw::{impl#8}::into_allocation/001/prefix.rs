// Answer 0

#[test]
fn test_into_allocation_empty_singleton() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;

    let table = RawTable::new_in(alloc);
    
    let result = table.into_allocation();
}

