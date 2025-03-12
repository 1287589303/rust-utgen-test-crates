// Answer 0

#[test]
fn test_build_hashes_inner_valid_case() {
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
            Ok(std::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    struct TestKey(i32);
    impl Hash for TestKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &TestKey) -> bool {
            self.0 == other.0
        }
    }

    let mut map: HashMap<TestKey, i32, TestHasher, TestAllocator> = HashMap {
        hash_builder: TestHasher,
        table: RawTable {
            table: Default::default(), // Place-holder, since it's not used in this test
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let keys = [&TestKey(1), &TestKey(2), &TestKey(3)];
    let result = map.build_hashes_inner(keys);

    let expected_length = keys.len();
    assert_eq!(result.len(), expected_length);
}

#[test]
fn test_build_hashes_inner_empty_case() {
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
            Ok(std::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    struct TestKey(i32);
    impl Hash for TestKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &TestKey) -> bool {
            self.0 == other.0
        }
    }

    let mut map: HashMap<TestKey, i32, TestHasher, TestAllocator> = HashMap {
        hash_builder: TestHasher,
        table: RawTable {
            table: Default::default(), // Place-holder, since it's not used in this test
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let keys: [&TestKey; 0] = [];
    let result = map.build_hashes_inner(keys);

    assert_eq!(result.len(), 0);
}

#[test]
#[should_panic]
fn test_build_hashes_inner_invalid_case() {
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
            Ok(std::ptr::NonNull::dangling())
        }
        unsafe fn deallocate(&self, _ptr: std::ptr::NonNull<u8>, _layout: std::alloc::Layout) {}
    }

    struct TestKey(i32);
    impl Hash for TestKey {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.hash(state);
        }
    }
    impl Equivalent<TestKey> for TestKey {
        fn equivalent(&self, other: &TestKey) -> bool {
            self.0 == other.0
        }
    }

    let mut map: HashMap<TestKey, i32, TestHasher, TestAllocator> = HashMap {
        hash_builder: TestHasher,
        table: RawTable {
            table: Default::default(), // Place-holder, since it's not used in this test
            alloc: TestAllocator,
            marker: PhantomData,
        },
    };

    let keys: [&TestKey; 1] = [&TestKey(0)]; // This should work, valid input would panics on panic condition only
    let _ = map.build_hashes_inner(keys);
}

