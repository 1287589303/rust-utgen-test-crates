// Answer 0

#[test]
fn test_get_mut_with_existing_entry() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut table = RawTable::new(TestAllocator);
    table.insert(("key1", 1));
    
    let mut entry = RawOccupiedEntryMut {
        elem: Bucket { ptr: NonNull::new(&mut table.get_mut("key1")).unwrap() },
        table: &mut table,
        hash_builder: &(),
    };

    let value_ref: &mut u32 = entry.get_mut();
}

#[test]
fn test_get_mut_with_multiple_entries() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut table = RawTable::new(TestAllocator);
    table.insert(("key2", 2));
    table.insert(("key3", 3));

    let mut entry = RawOccupiedEntryMut {
        elem: Bucket { ptr: NonNull::new(&mut table.get_mut("key2")).unwrap() },
        table: &mut table,
        hash_builder: &(),
    };

    let value_ref: &mut u32 = entry.get_mut();
}

#[test]
#[should_panic]
fn test_get_mut_with_non_existent_entry() {
    struct TestAllocator;

    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            unimplemented!()
        }
        
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
            unimplemented!()
        }
    }

    let mut table = RawTable::new(TestAllocator);
    table.insert(("key4", 4));

    let mut entry = RawOccupiedEntryMut {
        elem: Bucket { ptr: NonNull::new(&mut table.get_mut("key_non_existent")).unwrap() },
        table: &mut table,
        hash_builder: &(),
    };

    let value_ref: &mut u32 = entry.get_mut();
}

