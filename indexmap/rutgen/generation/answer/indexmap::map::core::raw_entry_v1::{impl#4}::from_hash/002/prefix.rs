// Answer 0

#[test]
fn test_from_hash_occupied_entry() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::Hasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<u64, String, TestHasher> = IndexMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());

    let hash: u64 = 1; // Hash corresponding to the key we want to find
    let key_match = |key: &u64| *key == 1;

    let builder = RawEntryBuilderMut { map: &mut map };
    let entry = builder.from_hash(hash, key_match); // Should return RawEntryMut::Occupied
}

#[test]
fn test_from_hash_occupied_entry_boundary() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::Hasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<u64, String, TestHasher> = IndexMap::new();
    map.insert(0, "zero".to_string());
    map.insert(u64::MAX, "max".to_string());

    let hash: u64 = 0; // Hash for the lowest boundary case
    let key_match = |key: &u64| *key == 0;

    let builder = RawEntryBuilderMut { map: &mut map };
    let entry = builder.from_hash(hash, key_match); // Should return RawEntryMut::Occupied

    let hash_max: u64 = u64::MAX;
    let key_match_max = |key: &u64| *key == u64::MAX;

    let entry_max = builder.from_hash(hash_max, key_match_max); // Should return RawEntryMut::Occupied
}

