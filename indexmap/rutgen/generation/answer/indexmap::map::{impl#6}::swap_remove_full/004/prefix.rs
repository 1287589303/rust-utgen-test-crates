// Answer 0

#[test]
fn test_swap_remove_full_empty_map() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }
    
    struct Key;
    struct Value;

    let mut map: IndexMap<Key, Value, TestHasher> = IndexMap::new();
    let key = Key;

    let result = map.swap_remove_full(&key);
    // No assertions, just calling the function with expected inputs
}

#[test]
fn test_swap_remove_full_non_existing_key() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    struct Key;
    struct Value;

    let mut map: IndexMap<Key, Value, TestHasher> = IndexMap::new();
    map.insert(Key, Value);  // Insert one item to ensure we don't match [x]
    
    let result = map.swap_remove_full(&Key);  // Key does not match
    // No assertions, just calling the function with expected inputs
}


