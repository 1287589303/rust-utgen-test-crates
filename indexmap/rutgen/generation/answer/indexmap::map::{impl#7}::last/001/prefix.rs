// Answer 0

#[test]
fn test_last_empty_indexmap() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: super::IndexMap<i32, i32, TestHasher> = super::IndexMap {
        core: super::IndexMapCore {
            indices: Default::default(),
            entries: Default::default(),
        },
        hash_builder: TestHasher,
    };
    
    let result = index_map.last();
}

#[test]
fn test_last_single_entry() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: super::IndexMap<i32, i32, TestHasher> = super::IndexMap {
        core: super::IndexMapCore {
            indices: Default::default(),
            entries: Default::default(),
        },
        hash_builder: TestHasher,
    };
    
    index_map.insert(1, 100);
    let result = index_map.last();
}

#[test]
fn test_last_multiple_entries() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: super::IndexMap<i32, i32, TestHasher> = super::IndexMap {
        core: super::IndexMapCore {
            indices: Default::default(),
            entries: Default::default(),
        },
        hash_builder: TestHasher,
    };
    
    index_map.insert(1, 100);
    index_map.insert(2, 200);
    index_map.insert(3, 300);
    let result = index_map.last();
}

#[test]
fn test_last_with_different_types() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: super::IndexMap<String, String, TestHasher> = super::IndexMap {
        core: super::IndexMapCore {
            indices: Default::default(),
            entries: Default::default(),
        },
        hash_builder: TestHasher,
    };

    index_map.insert("key1".to_string(), "value1".to_string());
    index_map.insert("key2".to_string(), "value2".to_string());
    let result = index_map.last();
}

#[test]
fn test_last_with_same_keys_different_values() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: super::IndexMap<i32, i32, TestHasher> = super::IndexMap {
        core: super::IndexMapCore {
            indices: Default::default(),
            entries: Default::default(),
        },
        hash_builder: TestHasher,
    };

    index_map.insert(1, 100);
    index_map.insert(1, 200); // This should update the value for key 1
    let result = index_map.last();
}

