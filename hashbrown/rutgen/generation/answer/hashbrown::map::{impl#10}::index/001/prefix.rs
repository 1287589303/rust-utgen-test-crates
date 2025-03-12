// Answer 0

#[test]
fn test_index_valid_key() {
    struct CustomAllocator;
    impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<&str, &str, DefaultHashBuilder, CustomAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::default(),
    };

    // Insert elements into the map
    // Note: RawTable will require implementation of insert methods that we are not providing here.
    // The focus is strictly on index retrieval for valid keys.
    // map.insert("a", "One");
    // map.insert("b", "Two");
    
    let _value_a = map[&"a"];
    let _value_b = map[&"b"];
}

#[test]
#[should_panic]
fn test_index_invalid_key() {
    struct CustomAllocator;
    impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }
    
    let map: HashMap<&str, &str, DefaultHashBuilder, CustomAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::default(),
    };

    let _value_invalid = map[&"does_not_exist"];
}

#[test]
fn test_index_empty_map() {
    struct CustomAllocator;
    impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let map: HashMap<&str, &str, DefaultHashBuilder, CustomAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::default(),
    };

    let _value_empty = map[&"non_existent_key"];
}

