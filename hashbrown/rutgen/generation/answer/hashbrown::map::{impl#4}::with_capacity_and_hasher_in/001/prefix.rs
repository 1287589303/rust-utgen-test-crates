// Answer 0

#[test]
fn test_with_zero_capacity_and_default_hash_builder() {
    let capacity = 0;
    let hash_builder = DefaultHashBuilder::default();
    let alloc = Global;
    let map: HashMap<usize, usize> = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, alloc);
}

#[test]
fn test_with_capacity_one_and_default_hash_builder() {
    let capacity = 1;
    let hash_builder = DefaultHashBuilder::default();
    let alloc = Global;
    let map: HashMap<usize, usize> = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, alloc);
}

#[test]
fn test_with_capacity_ten_and_default_hash_builder() {
    let capacity = 10;
    let hash_builder = DefaultHashBuilder::default();
    let alloc = Global;
    let map: HashMap<usize, usize> = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, alloc);
}

#[test]
fn test_with_zero_capacity_and_random_state() {
    let capacity = 0;
    let hash_builder = std::collections::hash_map::RandomState::new();
    let alloc = Global;
    let map: HashMap<usize, usize> = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, alloc);
}

#[test]
fn test_with_capacity_one_and_random_state() {
    let capacity = 1;
    let hash_builder = std::collections::hash_map::RandomState::new();
    let alloc = Global;
    let map: HashMap<usize, usize> = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, alloc);
}

#[test]
fn test_with_capacity_ten_and_random_state() {
    let capacity = 10;
    let hash_builder = std::collections::hash_map::RandomState::new();
    let alloc = Global;
    let map: HashMap<usize, usize> = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, alloc);
}

#[test]
fn test_with_capacity_five_and_custom_allocator() {
    struct CustomAllocator;

    unsafe impl Allocator for CustomAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }

    let capacity = 5;
    let hash_builder = DefaultHashBuilder::default();
    let alloc = CustomAllocator;
    let map: HashMap<usize, usize, DefaultHashBuilder, CustomAllocator> = HashMap::with_capacity_and_hasher_in(capacity, hash_builder, alloc);
}

