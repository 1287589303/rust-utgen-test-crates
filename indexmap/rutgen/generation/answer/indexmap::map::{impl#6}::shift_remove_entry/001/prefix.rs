// Answer 0

#[test]
fn test_shift_remove_entry_valid_key() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, &str, TestHasher> = IndexMap::default();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    let result = map.shift_remove_entry(&2);
}

#[test]
fn test_shift_remove_entry_valid_key_last() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, &str, TestHasher> = IndexMap::default();
    map.insert(1, "one");
    map.insert(2, "two");

    let result = map.shift_remove_entry(&2);
}

#[test]
fn test_shift_remove_entry_valid_key_first() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut map: IndexMap<i32, &str, TestHasher> = IndexMap::default();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");

    let result = map.shift_remove_entry(&1);
}

