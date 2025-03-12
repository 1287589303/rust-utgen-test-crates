// Answer 0

#[test]
fn test_or_insert_with_vacant_entry() {
    struct TestKey;
    struct TestValue(u32);
    
    impl Hash for TestKey {
        fn hash<H: core::hash::Hasher>(&self, _state: &mut H) {}
    }
    
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<TestKey, TestValue, DefaultHashBuilder, TestAllocator> = HashMap::new();
    let key = TestKey;
    let hash = 0u64;

    let entry_ref: EntryRef<TestKey, TestKey, TestValue, DefaultHashBuilder, TestAllocator> = 
        EntryRef::Vacant(VacantEntryRef { hash, key: &key, table: &mut map });

    let value_ref = entry_ref.or_insert_with(|| TestValue(5));
}

#[test]
fn test_or_insert_with_vacant_entry_multiple_calls() {
    struct TestKey;
    struct TestValue(u32);
    
    impl Hash for TestKey {
        fn hash<H: core::hash::Hasher>(&self, _state: &mut H) {}
    }

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<TestKey, TestValue, DefaultHashBuilder, TestAllocator> = HashMap::new();
    let key = TestKey;
    let hash = 0u64;

    let entry_ref: EntryRef<TestKey, TestKey, TestValue, DefaultHashBuilder, TestAllocator> = 
        EntryRef::Vacant(VacantEntryRef { hash, key: &key, table: &mut map });

    let first_value_ref = entry_ref.or_insert_with(|| TestValue(10));
    let second_value_ref = entry_ref.or_insert_with(|| TestValue(20));
}

#[test]
fn test_or_insert_with_vacant_entry_boundary_case() {
    struct TestKey;
    struct TestValue(u32);

    impl Hash for TestKey {
        fn hash<H: core::hash::Hasher>(&self, _state: &mut H) {}
    }

    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<TestKey, TestValue, DefaultHashBuilder, TestAllocator> = HashMap::new();
    let key = TestKey;
    let hash = 0u64;

    let entry_ref: EntryRef<TestKey, TestKey, TestValue, DefaultHashBuilder, TestAllocator> = 
        EntryRef::Vacant(VacantEntryRef { hash, key: &key, table: &mut map });

    let value_ref = entry_ref.or_insert_with(|| TestValue(0));
}

