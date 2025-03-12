// Answer 0

#[test]
fn test_with_capacity_and_hasher_in_zero_capacity() {
    struct MyAllocator;
    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(core::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }
    
    struct MyHasher;
    impl std::hash::BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = MyHasher;
    let alloc = MyAllocator;
    let set: hashbrown::HashSet<i32, MyHasher, MyAllocator> = hashbrown::HashSet::with_capacity_and_hasher_in(0, hasher, alloc);
}

#[test]
fn test_with_capacity_and_hasher_in_small_capacity() {
    struct MyAllocator;
    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(core::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }
    
    struct MyHasher;
    impl std::hash::BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = MyHasher;
    let alloc = MyAllocator;
    let set: hashbrown::HashSet<i32, MyHasher, MyAllocator> = hashbrown::HashSet::with_capacity_and_hasher_in(1, hasher, alloc);
}

#[test]
fn test_with_capacity_and_hasher_in_large_capacity() {
    struct MyAllocator;
    unsafe impl Allocator for MyAllocator {
        fn allocate(&self, _layout: core::alloc::Layout) -> Result<core::ptr::NonNull<u8>, ()> {
            Ok(core::ptr::NonNull::new_unchecked(core::ptr::null_mut()))
        }
        unsafe fn deallocate(&self, _ptr: core::ptr::NonNull<u8>, _layout: core::alloc::Layout) {}
    }
    
    struct MyHasher;
    impl std::hash::BuildHasher for MyHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let hasher = MyHasher;
    let alloc = MyAllocator;
    let set: hashbrown::HashSet<i32, MyHasher, MyAllocator> = hashbrown::HashSet::with_capacity_and_hasher_in(1000, hasher, alloc);
}

