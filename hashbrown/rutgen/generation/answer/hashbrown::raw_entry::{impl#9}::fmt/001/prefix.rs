// Answer 0

#[test]
fn test_raw_entry_builder_mut_fmt_empty_map() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<i32, &str, DefaultHasher, CustomAllocator> = HashMap {
        hash_builder: DefaultHasher::new(),
        table: RawTable::new(),
    };

    let mut formatter = core::fmt::Formatter::new();
    
    let builder = RawEntryBuilderMut { map: &mut map };
    let _ = builder.fmt(&mut formatter);
}

#[test]
fn test_raw_entry_builder_mut_fmt_full_map() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;

    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<i32, &str, DefaultHasher, CustomAllocator> = HashMap {
        hash_builder: DefaultHasher::new(),
        table: RawTable::new(),
    };

    // Fill the map to create a "full" state
    for i in 0..10 {
        map.table.insert((i, "test"));
    }

    let mut formatter = core::fmt::Formatter::new();
    
    let builder = RawEntryBuilderMut { map: &mut map };
    let _ = builder.fmt(&mut formatter);
}

#[test]
fn test_raw_entry_builder_mut_fmt_with_collision() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    struct CustomHasher {
        value: u64,
    }

    impl Default for CustomHasher {
        fn default() -> Self {
            Self { value: 0 }
        }
    }

    impl Hasher for CustomHasher {
        fn finish(&self) -> u64 {
            self.value
        }

        fn write(&mut self, _bytes: &[u8]) {
            self.value += 1; // Simulate collision by incrementing for each call
        }
    }

    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut map: HashMap<i32, &str, CustomHasher, CustomAllocator> = HashMap {
        hash_builder: CustomHasher::default(),
        table: RawTable::new(),
    };

    // Simulate hash collisions
    let colliding_keys = vec![1, 2]; // Assuming these collide
    for key in colliding_keys {
        map.table.insert((key, "collide"));
    }

    let mut formatter = core::fmt::Formatter::new();
    
    let builder = RawEntryBuilderMut { map: &mut map };
    let _ = builder.fmt(&mut formatter);
}

