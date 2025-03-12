// Answer 0

#[test]
fn test_bitxor_with_non_empty_sets_same_hash_builder() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set1: IndexSet<i32, DummyHasher> = IndexSet::default();
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, DummyHasher> = IndexSet::default();
    set2.insert(2);
    set2.insert(3);

    let _result = &set1 ^ &set2;
}

#[test]
fn test_bitxor_with_non_empty_sets_different_hash_builder() {
    struct AnotherDummyHasher;
    impl BuildHasher for AnotherDummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set1: IndexSet<i32, DummyHasher> = IndexSet::default();
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, AnotherDummyHasher> = IndexSet::default();
    set2.insert(2);
    set2.insert(3);

    let _result = &set1 ^ &set2;
}

#[test]
fn test_bitxor_with_empty_first_set() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set1: IndexSet<i32, DummyHasher> = IndexSet::default(); // Empty set

    let mut set2: IndexSet<i32, DummyHasher> = IndexSet::default();
    set2.insert(2);
    set2.insert(3);

    let _result = &set1 ^ &set2;
}

#[test]
fn test_bitxor_with_empty_second_set() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set1: IndexSet<i32, DummyHasher> = IndexSet::default();
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, DummyHasher> = IndexSet::default(); // Empty set

    let _result = &set1 ^ &set2;
}

#[test]
fn test_bitxor_with_disjoint_sets() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set1: IndexSet<i32, DummyHasher> = IndexSet::default();
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, DummyHasher> = IndexSet::default();
    set2.insert(3);
    set2.insert(4);

    let _result = &set1 ^ &set2;
}

#[test]
fn test_bitxor_with_identical_sets() {
    struct DummyHasher;
    impl BuildHasher for DummyHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut set1: IndexSet<i32, DummyHasher> = IndexSet::default();
    set1.insert(1);
    set1.insert(2);

    let mut set2: IndexSet<i32, DummyHasher> = IndexSet::default();
    set2.insert(1);
    set2.insert(2);

    let _result = &set1 ^ &set2;
}

