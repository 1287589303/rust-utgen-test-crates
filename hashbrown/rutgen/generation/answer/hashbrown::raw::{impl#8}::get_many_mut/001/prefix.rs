// Answer 0

#[test]
fn test_get_many_mut_with_duplicates() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(10, TestAllocator);
    
    // Assume some values have been inserted into the table, using distinct hash values
    let _ = table.insert(1, 10, |x| *x);
    let _ = table.insert(2, 20, |x| *x);
    let _ = table.insert(3, 30, |x| *x);

    // Create an array of hashes with duplicates and at least one missing hash
    let hashes = [1, 2, 1, 4]; // 1 and 2 are valid, but 4 does not exist

    unsafe {
        let _result = table.get_many_mut(hashes, |i, k| {
            match i {
                0 => *k == 10,
                1 => *k == 20,
                _ => false, // Only return true for valid indices
            }
        });
    }
}

#[test]
fn test_get_many_mut_with_all_duplicates() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(10, TestAllocator);
    
    // Insert a couple of values
    let _ = table.insert(1, 10, |x| *x);
    let _ = table.insert(2, 20, |x| *x);

    // Create an array with all hashes being duplicates
    let hashes = [1, 1]; // All duplicates

    unsafe {
        let _result = table.get_many_mut(hashes, |i, k| {
            *k == 10
        });
    }
}

#[test]
fn test_get_many_mut_with_only_missing() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }

    let mut table: RawTable<u32, TestAllocator> = RawTable::with_capacity_in(10, TestAllocator);
    
    // Insert some values
    let _ = table.insert(1, 10, |x| *x);
    let _ = table.insert(2, 20, |x| *x);

    // Create an array where all hashes are missing
    let hashes = [3, 4]; // Both missing

    unsafe {
        let _result = table.get_many_mut(hashes, |i, k| {
            *k == 30 // no corresponding key
        });
    }
}

