// Answer 0

#[test]
fn test_build_hashes_inner_empty_array() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    struct TestAllocator;

    unsafe impl Allocator for TestAllocator {
        fn allocate(&self, _layout: std::alloc::Layout) -> Result<std::ptr::NonNull<u8>, ()> {
            unimplemented!()
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {
            unimplemented!()
        }
    }

    let mut hashmap: HashMap<i32, i32, TestHasher, TestAllocator> = HashMap {
        hash_builder: TestHasher,
        table: RawTable {
            table: RawTableInner,
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let hashes: [u64; 0] = hashmap.build_hashes_inner(&[]);
}

