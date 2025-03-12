// Answer 0

#[test]
fn test_entry_vacant_debug_tuple() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) { }
    }

    let allocator = TestAllocator;
    let mut hashmap: HashMap<&str, i32, DefaultHashBuilder, _> = HashMap::with_hasher(allocator);
    let key: &str = "test";

    let vacant_entry = Entry::Vacant(VacantEntry {
        hash: 12345,
        key: key,
        table: &mut hashmap,
    });

    let mut formatter = std::fmt::Formatter::new();
    vacant_entry.fmt(&mut formatter);
}

#[test]
fn test_entry_occupied_debug_tuple() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::new_unchecked(std::ptr::null_mut()))
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) { }
    }

    let allocator = TestAllocator;
    let mut hashmap: HashMap<&str, i32, DefaultHashBuilder, _> = HashMap::with_hasher(allocator);
    hashmap.insert("key", 42);

    let occupied_entry = Entry::Occupied(OccupiedEntry {
        hash: 12345,
        elem: Bucket((String::from("key"), 42)),
        table: &mut hashmap,
    });

    let mut formatter = std::fmt::Formatter::new();
    occupied_entry.fmt(&mut formatter);
}

