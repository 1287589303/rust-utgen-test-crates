// Answer 0

#[test]
fn test_shift_remove_existing_key() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }
    
    let mut map: IndexMap<i32, String, TestHasher> = IndexMap::new();
    map.insert(1, "A".to_string());
    map.insert(2, "B".to_string());
    
    let value = map.shift_remove(&1);
    let remaining_value = map.get(&2);
}

#[test]
fn test_shift_remove_non_existing_key() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }
    
    let mut map: IndexMap<i32, String, TestHasher> = IndexMap::new();
    map.insert(1, "A".to_string());
    
    let value = map.shift_remove(&2);
}

#[test]
fn test_shift_remove_empty_map() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }
    
    let mut map: IndexMap<i32, String, TestHasher> = IndexMap::new();
    
    let value = map.shift_remove(&1);
}

#[test]
fn test_shift_remove_multiple_entries() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            RandomState::new()
        }
    }
    
    let mut map: IndexMap<i32, String, TestHasher> = IndexMap::new();
    map.insert(1, "A".to_string());
    map.insert(2, "B".to_string());
    map.insert(3, "C".to_string());
    
    let value = map.shift_remove(&2);
    let remaining_value = map.get(&1);
    let remaining_value_2 = map.get(&3);
}

