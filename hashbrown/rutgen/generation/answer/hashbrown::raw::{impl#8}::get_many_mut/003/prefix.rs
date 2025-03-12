// Answer 0

#[test]
fn test_get_many_mut_single_unique_hash() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(1, 1).unwrap())))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(1, TestAllocator);
    unsafe {
        table.insert(1, 10, |x| *x);
    }

    let hashes = [1u64];
    let result = table.get_many_mut(hashes, |_, _| false);
    let _ = result; // Consume the result
}

#[test]
fn test_get_many_mut_multiple_unique_hashes() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(1, 1).unwrap())))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(4, TestAllocator);
    unsafe {
        table.insert(1, 10, |x| *x);
        table.insert(2, 20, |x| *x);
        table.insert(3, 30, |x| *x);
    }

    let hashes = [1u64, 2u64, 3u64];
    let result = table.get_many_mut(hashes, |_, _| false);
    let _ = result; // Consume the result
}

#[test]
fn test_get_many_mut_boundary_hashes() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(1, 1).unwrap())))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let max_u64 = u64::MAX;
    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(2, TestAllocator);
    unsafe {
        table.insert(max_u64, 99, |x| *x);
        table.insert(max_u64 - 1, 88, |x| *x);
    }

    let hashes = [max_u64, max_u64 - 1];
    let result = table.get_many_mut(hashes, |_, _| false);
    let _ = result; // Consume the result
}

#[test]
fn test_get_many_mut_no_entries_found() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(1, 1).unwrap())))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(1, TestAllocator);

    let hashes = [99u64];
    let result = table.get_many_mut(hashes, |_, _| false);
    let _ = result; // Consume the result
}

#[test]
fn test_get_many_mut_with_duplicates() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::alloc::alloc(Layout::from_size_align(1, 1).unwrap())))
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(10, TestAllocator);
    unsafe {
        table.insert(1, 10, |x| *x);
        table.insert(2, 20, |x| *x);
        table.insert(3, 30, |x| *x);
    }

    let hashes = [1u64, 1u64]; // Duplicate hashes
    let result = table.get_many_mut(hashes, |_, _| false);
    let _ = result; // Consume the result
}

