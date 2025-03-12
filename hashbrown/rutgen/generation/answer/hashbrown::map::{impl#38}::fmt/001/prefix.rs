// Answer 0

#[test]
fn test_vacant_entry_debug() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut hashmap: HashMap<i32, &str, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };
    
    let vacant_entry = VacantEntry {
        hash: 1234,
        key: 1,
        table: &mut hashmap,
    };

    let mut buffer = String::new();
    vacant_entry.fmt(&mut buffer);
}

#[test]
fn test_vacant_entry_debug_with_diff_key_type() {
    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut hashmap: HashMap<String, i32, DefaultHashBuilder, TestAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    let vacant_entry = VacantEntry {
        hash: 5678,
        key: String::from("test"),
        table: &mut hashmap,
    };

    let mut buffer = String::new();
    vacant_entry.fmt(&mut buffer);
}

