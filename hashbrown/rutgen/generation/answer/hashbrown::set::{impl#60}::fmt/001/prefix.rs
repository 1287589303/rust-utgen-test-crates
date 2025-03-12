// Answer 0

#[test]
fn test_occupied_entry_debug_empty_entry() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table = HashMap::with_capacity_and_hasher(1, DefaultHashBuilder::default());
    let entry = OccupiedEntry {
        hash: 0,
        elem: Bucket::default(),
        table: &mut table,
    };

    let mut formatter = fmt::Formatter::new();
    let _ = entry.fmt(&mut formatter);
}

#[test]
fn test_occupied_entry_debug_valid_entry() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table = HashMap::with_capacity_and_hasher(1, DefaultHashBuilder::default());
    table.insert("key", "value");

    let entry = OccupiedEntry {
        hash: 1,
        elem: Bucket::new(("key", "value")),
        table: &mut table,
    };

    let mut formatter = fmt::Formatter::new();
    let _ = entry.fmt(&mut formatter);
}

#[test]
fn test_occupied_entry_debug_boundary_case() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let allocator = TestAllocator;
    let mut table = HashMap::with_capacity_and_hasher(1, DefaultHashBuilder::default());

    let entry = OccupiedEntry {
        hash: 0,
        elem: Bucket::new(("boundary_key", "boundary_value")),
        table: &mut table,
    };

    let mut formatter = fmt::Formatter::new();
    let _ = entry.fmt(&mut formatter);
}

