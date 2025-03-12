// Answer 0

#[test]
fn test_hasher_with_integer_keys_values() {
    let hasher = DefaultHashBuilder::default();
    let mut map: HashMap<i32, i32> = HashMap::with_capacity_and_hasher_in(10, hasher, Global);
    map.insert(1, 10);
    map.insert(2, 20);
    let returned_hasher: &DefaultHashBuilder = map.hasher();
}

#[test]
fn test_hasher_with_string_keys_integer_values() {
    let hasher = DefaultHashBuilder::default();
    let mut map: HashMap<String, i32> = HashMap::with_capacity_and_hasher_in(5, hasher, Global);
    map.insert("key1".to_string(), 10);
    map.insert("key2".to_string(), 20);
    let returned_hasher: &DefaultHashBuilder = map.hasher();
}

#[test]
fn test_hasher_with_integer_keys_string_values() {
    let hasher = DefaultHashBuilder::default();
    let mut map: HashMap<i32, String> = HashMap::with_capacity_and_hasher_in(8, hasher, Global);
    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());
    let returned_hasher: &DefaultHashBuilder = map.hasher();
}

#[test]
fn test_hasher_with_custom_allocator() {
    struct CustomAllocator;
    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<NonNull<u8>, ()> {
            // Implementation detail omitted for brevity
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: std::alloc::Layout) {
            // Implementation detail omitted for brevity
            unimplemented!()
        }
    }

    let hasher = DefaultHashBuilder::default();
    let alloc = CustomAllocator;
    let mut map: HashMap<i32, i32, DefaultHashBuilder, CustomAllocator> =
        HashMap::with_capacity_and_hasher_in(10, hasher, alloc);
    map.insert(1, 10);
    map.insert(2, 20);
    let returned_hasher: &DefaultHashBuilder = map.hasher();
}

