// Answer 0

#[test]
fn test_get_many_mut_single_entry() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(1, TestAllocator);
    let hash = 123u64;
    table.insert(hash, 42, |v| *v);
    let result = table.get_many_mut::<1>([hash], |_, k| *k == 42);
    let _ = result; // Use the result to ensure the function is compiled
}

#[test]
fn test_get_many_mut_multiple_entries() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(2, TestAllocator);
    let hashes = [123u64, 456u64];
    table.insert(hashes[0], 42, |v| *v);
    table.insert(hashes[1], 84, |v| *v);
    let result = table.get_many_mut::<2>(hashes, |_, k| *k == 42 || *k == 84);
    let _ = result; // Use the result to ensure the function is compiled
}

#[test]
fn test_get_many_mut_boundary_cases() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(3, TestAllocator);
    let hashes = [123u64, 456u64, 789u64];
    table.insert(hashes[0], 42, |v| *v);
    table.insert(hashes[1], 84, |v| *v);
    table.insert(hashes[2], 126, |v| *v);
    let result = table.get_many_mut::<3>(hashes, |_, k| *k == 42 || *k == 84 || *k == 126);
    let _ = result; // Use the result to ensure the function is compiled
} 

#[test]
#[should_panic]
fn test_get_many_mut_duplicate_hashes() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> { Ok(NonNull::dangling()) }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let mut table: RawTable<u64, TestAllocator> = RawTable::with_capacity_in(2, TestAllocator);
    let hash = 123u64;
    table.insert(hash, 42, |v| *v);
    table.insert(hash, 84, |v| *v); // Inserting duplicate hash
    let _ = table.get_many_mut::<2>([hash, hash], |_, k| *k == 42 || *k == 84); // Should panic
}

