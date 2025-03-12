// Answer 0

#[test]
fn test_get_full_mut2_existing_key() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: IndexMap<i32, String, TestHasher> = IndexMap::new(TestHasher);
    index_map.insert(1, "Value1".to_string());

    let key = 1;
    let result = index_map.get_full_mut2(&key);
    // result is expected to be Some((0, &mut "Value1")) since 1 is a valid key.
}

#[test]
fn test_get_full_mut2_existing_key_multiple_entries() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: IndexMap<i32, String, TestHasher> = IndexMap::new(TestHasher);
    index_map.insert(1, "Value1".to_string());
    index_map.insert(2, "Value2".to_string());

    let key = 2;
    let result = index_map.get_full_mut2(&key);
    // result is expected to be Some((1, &mut "Value2")) since 2 exists in the map.
}

#[test]
fn test_get_full_mut2_existing_key_boundary() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_map: IndexMap<i32, String, TestHasher> = IndexMap::new(TestHasher);
    index_map.insert(0, "Value0".to_string());
    index_map.insert(99, "Value99".to_string());

    let key = 99;
    let result = index_map.get_full_mut2(&key);
    // result is expected to be Some((1, &mut "Value99")) since 99 exists as the last entry.
}

