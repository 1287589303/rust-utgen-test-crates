// Answer 0

#[test]
fn test_buckets_zero_mask() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }
    
    let alloc = TestAllocator;
    let mut table = RawTable::new_in(alloc);
    table.table.bucket_mask = 0; // Setting bucket_mask for test case
    let _ = table.buckets(); // Expected: 1
}

#[test]
fn test_buckets_one_mask() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }
    
    let alloc = TestAllocator;
    let mut table = RawTable::new_in(alloc);
    table.table.bucket_mask = 1; // Setting bucket_mask for test case
    let _ = table.buckets(); // Expected: 2
}

#[test]
fn test_buckets_two_mask() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }
    
    let alloc = TestAllocator;
    let mut table = RawTable::new_in(alloc);
    table.table.bucket_mask = 2; // Setting bucket_mask for test case
    let _ = table.buckets(); // Expected: 3
}

#[test]
fn test_buckets_not_power_of_two_mask() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }
    
    let alloc = TestAllocator;
    let mut table = RawTable::new_in(alloc);
    table.table.bucket_mask = 3; // Setting bucket_mask for test case (not a power of two)
    let _ = table.buckets(); // Expected: 4
}

#[test]
fn test_buckets_large_mask() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }
    
    let alloc = TestAllocator;
    let mut table = RawTable::new_in(alloc);
    table.table.bucket_mask = 15; // Setting bucket_mask for test case
    let _ = table.buckets(); // Expected: 16
}

#[test]
fn test_buckets_max_usize_mask() {
    struct TestAllocator;
    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _: NonNull<u8>, _: Layout) {}
    }
    
    let alloc = TestAllocator;
    let mut table = RawTable::new_in(alloc);
    table.table.bucket_mask = usize::MAX; // Setting bucket_mask for test case
    let _ = table.buckets(); // Expected: usize::MAX + 1
}

