// Answer 0

#[test]
fn test_with_hasher_in_default_allocator() {
    struct TestHasher;
    
    impl BuildHasher for TestHasher {
        fn build_hasher(&self) -> std::hash::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let hasher = TestHasher;
    let allocator = Global;
    let set: HashSet<i32, TestHasher, Global> = HashSet::with_hasher_in(hasher, allocator);
}

#[test]
fn test_with_hasher_in_custom_allocator() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        fn build_hasher(&self) -> std::hash::Hasher {
            std::collections::hash_map::DefaultHasher::new()
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

    let hasher = TestHasher;
    let allocator = CustomAllocator;
    let set: HashSet<i32, TestHasher, CustomAllocator> = HashSet::with_hasher_in(hasher, allocator);
}

#[test]
fn test_with_hasher_in_default_hash_builder() {
    let hasher = DefaultHashBuilder::default();
    let allocator = Global;
    let set: HashSet<i32, DefaultHashBuilder, Global> = HashSet::with_hasher_in(hasher, allocator);
}

