// Answer 0

#[test]
fn test_into_keys_non_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::default::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut map: IndexMap<i32, &str, TestHasher> = IndexMap::with_capacity_and_hasher(10, TestHasher);
    map.insert(1, "one");
    map.insert(2, "two");
    
    let keys_iter = map.into_keys();
}

#[test]
fn test_into_keys_multiple_entries() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::default::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut map: IndexMap<i32, &str, TestHasher> = IndexMap::with_capacity_and_hasher(10, TestHasher);
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    
    let keys_iter = map.into_keys();
}

#[test]
fn test_into_keys_single_entry() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::hash::rustc_hash::default::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            Self::Hasher::default()
        }
    }

    let mut map: IndexMap<i32, &str, TestHasher> = IndexMap::with_capacity_and_hasher(10, TestHasher);
    map.insert(5, "five");
    
    let keys_iter = map.into_keys();
}

