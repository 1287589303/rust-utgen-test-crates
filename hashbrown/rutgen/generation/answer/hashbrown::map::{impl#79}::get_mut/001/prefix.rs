// Answer 0

#[test]
fn test_get_mut_with_existing_entry() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("poneyland", 12);
    
    if let OccupiedEntry { elem: Bucket { ptr }, .. } = map.entry("poneyland").or_insert(12) {
        let mut entry = OccupiedEntry {
            hash: 0,
            elem: Bucket { ptr },
            table: &mut map,
        };

        unsafe {
            let val_mut: &mut u32 = entry.get_mut();
            *val_mut += 10;
        }
    }
}

#[test]
fn test_get_mut_with_multiple_operations() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("poneyland", 12);
    
    if let OccupiedEntry { elem: Bucket { ptr }, .. } = map.entry("poneyland").or_insert(12) {
        let mut entry = OccupiedEntry {
            hash: 0,
            elem: Bucket { ptr },
            table: &mut map,
        };

        unsafe {
            for _ in 0..3 {
                let val_mut: &mut u32 = entry.get_mut();
                *val_mut += 5;
            }
        }
    }
}

#[test]
fn test_get_mut_boundary_case() {
    struct TestAllocator;
    
    impl Allocator for TestAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Ok(NonNull::dangling())
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut map: HashMap<&str, u32, DefaultHashBuilder, TestAllocator> = HashMap::new();
    map.insert("poneyland", 0); // boundary case with initial value of 0
    
    if let OccupiedEntry { elem: Bucket { ptr }, .. } = map.entry("poneyland").or_insert(0) {
        let mut entry = OccupiedEntry {
            hash: 0,
            elem: Bucket { ptr },
            table: &mut map,
        };

        unsafe {
            let val_mut: &mut u32 = entry.get_mut();
            *val_mut += 1; // should still be safe as it is occupied.
        }
    }
}

