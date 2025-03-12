// Answer 0

#[test]
fn test_last_entry_empty_map() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::DummyHasher
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::with_hasher(TestHasher);
    let entry = map.last_entry();
}

#[test]
fn test_last_entry_single_entry() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::hash::DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::hash::DummyHasher
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::with_hasher(TestHasher);
    map.insert(1, 10);
    let entry = map.last_entry();
}

