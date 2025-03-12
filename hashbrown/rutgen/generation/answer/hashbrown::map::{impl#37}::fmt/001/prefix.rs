// Answer 0

#[test]
fn test_occupied_entry_debug_with_integer_key_value() {
    struct SimpleAllocator;
    unsafe impl Allocator for SimpleAllocator {
        // Implement allocate and deallocate methods as no-ops for testing
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let mut hashmap: HashMap<i32, i32, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };
    
    let occupied_entry = OccupiedEntry {
        hash: 0,
        elem: Bucket { ptr: NonNull::dangling() },
        table: &mut hashmap,
    };

    let _ = format!("{:?}", occupied_entry);
}

#[test]
fn test_occupied_entry_debug_with_string_key_value() {
    struct SimpleAllocator;
    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }
    
    let mut hashmap: HashMap<String, String, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };
    
    let occupied_entry = OccupiedEntry {
        hash: 0,
        elem: Bucket { ptr: NonNull::dangling() },
        table: &mut hashmap,
    };

    let _ = format!("{:?}", occupied_entry);
}

#[test]
fn test_occupied_entry_debug_with_custom_struct() {
    #[derive(Debug)]
    struct Key {
        id: i32,
    }

    #[derive(Debug)]
    struct Value {
        name: String,
    }

    struct SimpleAllocator;
    unsafe impl Allocator for SimpleAllocator {
        fn allocate(&self, _layout: Layout) -> Result<NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}
    }

    let mut hashmap: HashMap<Key, Value, DefaultHashBuilder, SimpleAllocator> = HashMap {
        hash_builder: DefaultHashBuilder::new(),
        table: RawTable::new(),
    };

    let occupied_entry = OccupiedEntry {
        hash: 0,
        elem: Bucket { ptr: NonNull::dangling() },
        table: &mut hashmap,
    };

    let _ = format!("{:?}", occupied_entry);
}

