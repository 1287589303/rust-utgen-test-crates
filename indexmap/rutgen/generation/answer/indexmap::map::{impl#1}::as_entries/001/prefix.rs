// Answer 0

#[test]
fn test_as_entries_non_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut index_map: IndexMap<i32, String, TestHasher> = IndexMap::default();
    index_map.core.entries.push(Bucket { hash: 0.into(), key: 1, value: "One".to_string() });
    index_map.core.entries.push(Bucket { hash: 0.into(), key: 2, value: "Two".to_string() });

    let entries = index_map.as_entries();
    let _ = entries; // Using the entries to ensure the function is invoked
}

#[test]
fn test_as_entries_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let index_map: IndexMap<i32, String, TestHasher> = IndexMap::default();

    let entries = index_map.as_entries();
    let _ = entries; // Using the entries to ensure the function is invoked
}

