// Answer 0

#[test]
fn test_allocator_with_global_allocator() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = TestAllocator;
    let table: RawTable<u32, TestAllocator> = RawTable::new_in(alloc);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_with_custom_allocator() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = CustomAllocator;
    let table: RawTable<u64, CustomAllocator> = RawTable::new_in(alloc);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_with_capacity() {
    let alloc = Global;
    let table: RawTable<i32, Global> = RawTable::with_capacity_in(10, alloc);
    let allocator = table.allocator();
}

#[test]
fn test_allocator_with_fail_case() {
    struct FailingAllocator;

    unsafe impl Allocator for FailingAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let alloc = FailingAllocator;
    let table: RawTable<u16, FailingAllocator> = RawTable::new_in(alloc);
    let allocator = table.allocator();
}

