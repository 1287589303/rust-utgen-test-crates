// Answer 0

#[test]
fn test_allocation_size_or_zero_empty_singleton() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        // Implement required methods for the Allocator trait
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout { size: 0, ctrl_align: 1 };
    let raw_table = RawTableInner {
        bucket_mask: 0,
        ctrl: NonNull::new_unchecked(std::ptr::null_mut()),
        growth_left: 0,
        items: 0,
    };

    let size = unsafe { raw_table.allocation_size_or_zero(table_layout) };
}

#[test]
fn test_allocation_size_or_zero_non_empty_table() {
    struct TestAllocator;
    impl Allocator for TestAllocator {
        // Implement required methods for the Allocator trait
    }

    let alloc = TestAllocator;
    let table_layout = TableLayout { size: 8, ctrl_align: 4 };
    let raw_table = RawTableInner {
        bucket_mask: 1,
        ctrl: NonNull::new_unchecked(std::ptr::null_mut()),
        growth_left: 0,
        items: 0,
    };

    let size = unsafe { raw_table.allocation_size_or_zero(table_layout) };
}

