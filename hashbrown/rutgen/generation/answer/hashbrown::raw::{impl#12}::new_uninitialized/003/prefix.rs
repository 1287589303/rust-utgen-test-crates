// Answer 0

#[test]
fn test_new_uninitialized_buckets_exceeding_capacity() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout::new::<u8>();
    let buckets = 1024 * 1024; // This value exceeds typical maximums for capacity checks.

    let result = RawTableInner::new_uninitialized(&alloc, table_layout, buckets, Fallibility::Fallible);
}

#[test]
fn test_new_uninitialized_zero_buckets() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout::new::<u8>();
    let buckets = 0; // Not a power of two but will test for a precondition check

    let result = RawTableInner::new_uninitialized(&alloc, table_layout, buckets, Fallibility::Fallible);
}

#[test]
fn test_new_uninitialized_large_power_of_two_buckets() {
    struct MockAllocator;

    unsafe impl Allocator for MockAllocator {
        fn allocate(&self, _: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }
    }

    let alloc = MockAllocator;
    let table_layout = TableLayout::new::<u8>();
    let buckets = usize::MAX / 2; // A large power of two that may not fit capacity checks.

    let result = RawTableInner::new_uninitialized(&alloc, table_layout, buckets, Fallibility::Fallible);
}

