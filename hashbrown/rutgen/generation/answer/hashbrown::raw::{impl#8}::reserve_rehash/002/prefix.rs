// Answer 0

#[test]
fn test_reserve_rehash_with_zero_additional() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<u8, TestAllocator> = RawTable::new_in(allocator);
    let additional = 0;
    let hasher = |x: &u8| *x as u64;
    let fallibility = Fallibility::Fallible;

    unsafe {
        table.table.reserve_rehash(additional, hasher, fallibility).unwrap();
    }
}

#[test]
fn test_reserve_rehash_with_max_additional() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<u8, TestAllocator> = RawTable::new_in(allocator);
    let additional = isize::MAX as usize;
    let hasher = |x: &u8| *x as u64;
    let fallibility = Fallibility::Infallible;

    unsafe {
        table.table.reserve_rehash(additional, hasher, fallibility).unwrap();
    }
}

#[test]
fn test_reserve_rehash_with_some_additional() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<u8, TestAllocator> = RawTable::new_in(allocator);
    let additional = 10;
    let hasher = |x: &u8| *x as u64;
    let fallibility = Fallibility::Fallible;

    unsafe {
        table.table.reserve_rehash(additional, hasher, fallibility).unwrap();
    }
}

#[test]
fn test_reserve_rehash_with_fallible_behavior() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table: RawTable<u8, TestAllocator> = RawTable::new_in(allocator);
    let additional = 5;
    let hasher = |x: &u8| *x as u64;
    let fallibility = Fallibility::Fallible;

    unsafe {
        table.table.reserve_rehash(additional, hasher, fallibility).unwrap();
    }
}

