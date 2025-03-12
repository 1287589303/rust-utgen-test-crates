// Answer 0

#[test]
fn test_get_full_mut_existing_key() {
    struct TestHasher;
    
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHasher> = IndexMap::new();
    map.insert(1, "value1".to_string());
    map.insert(2, "value2".to_string());

    let key_ref = &1;
    let result = map.get_full_mut(key_ref);
}

#[test]
fn test_get_full_mut_another_existing_key() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHasher> = IndexMap::new();
    map.insert(3, "value3".to_string());
    map.insert(4, "value4".to_string());

    let key_ref = &4;
    let result = map.get_full_mut(key_ref);
}

#[test]
fn test_get_full_mut_with_mutate() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHasher> = IndexMap::new();
    map.insert(5, "value5".to_string());

    let key_ref = &5;
    let result = map.get_full_mut(key_ref);
}

