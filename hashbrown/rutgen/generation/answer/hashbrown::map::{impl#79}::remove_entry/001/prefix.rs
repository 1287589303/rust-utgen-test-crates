// Answer 0

#[test]
fn test_remove_entry_existing_key_value() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut())) // Stub implementation
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("test_key", 42);

    if let OccupiedEntry { key, elem, table } = map.entry("test_key").or_insert(42) {
        let result = OccupiedEntry { hash: 0, elem, table }.remove_entry();
    }
}

#[test]
fn test_remove_entry_after_insert() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut())) // Stub implementation
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("example", 100);

    if let OccupiedEntry { key, elem, table } = map.entry("example").or_insert(100) {
        let result = OccupiedEntry { hash: 0, elem, table }.remove_entry();
    }
}

#[test]
fn test_remove_entry_with_multiple_entries() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut())) // Stub implementation
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);

    if let OccupiedEntry { key, elem, table } = map.entry("key1").or_insert(1) {
        let result = OccupiedEntry { hash: 0, elem, table }.remove_entry();
    }

    if let OccupiedEntry { key, elem, table } = map.entry("key2").or_insert(2) {
        let result = OccupiedEntry { hash: 0, elem, table }.remove_entry();
    }
}

