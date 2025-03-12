// Answer 0

#[test]
fn test_default_hashmap_with_integer_key_and_string_value() {
    let map: HashMap<u32, String> = Default::default();
    let _ = map.capacity();
}

#[test]
fn test_default_hashmap_with_integer_key_and_custom_hasher() {
    use std::collections::hash_map::RandomState;
    let map: HashMap<u32, String, RandomState> = HashMap::default();
    let _ = map.capacity();
}

#[test]
fn test_default_hashmap_with_string_key_and_integer_value() {
    let map: HashMap<String, u32> = Default::default();
    let _ = map.capacity();
}

#[test]
fn test_default_hashmap_with_string_key_and_custom_hasher() {
    use std::collections::hash_map::RandomState;
    let map: HashMap<String, u32, RandomState> = HashMap::default();
    let _ = map.capacity();
}

#[test]
fn test_default_hashmap_with_custom_allocator() {
    struct CustomAllocator;
    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Err(())
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let map: HashMap<u32, String, DefaultHashBuilder, CustomAllocator> = HashMap::default();
    let _ = map.capacity();
}

