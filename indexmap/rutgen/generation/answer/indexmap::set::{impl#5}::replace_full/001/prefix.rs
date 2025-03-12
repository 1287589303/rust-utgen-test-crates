// Answer 0

#[test]
fn test_replace_full_existing_value() {
    struct DummyHasher; // A struct to serve as a hasher
    impl BuildHasher for DummyHasher {
        type Hasher = DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher
        }
    }

    impl std::hash::Hasher for DummyHasher {
        fn finish(&self) -> u64 {
            0 // Dummy implementation
        }

        fn write(&mut self, _: &[u8]) {
            // Dummy implementation
        }
    }

    #[derive(Hash, Eq, PartialEq, Clone)]
    struct TestValue(i32);

    let mut set: IndexSet<TestValue, DummyHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore {
                indices: Indices::new(),
                entries: Entries::new(),
            },
            hash_builder: DummyHasher,
        },
    };

    set.insert(TestValue(42));
    let (index, replaced) = set.replace_full(TestValue(42));

    let expected_replaced_value = Some(TestValue(42));
}

#[test]
fn test_replace_full_with_new_value() {
    struct DummyHasher; // A struct to serve as a hasher
    impl BuildHasher for DummyHasher {
        type Hasher = DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            DummyHasher
        }
    }

    impl std::hash::Hasher for DummyHasher {
        fn finish(&self) -> u64 {
            0 // Dummy implementation
        }

        fn write(&mut self, _: &[u8]) {
            // Dummy implementation
        }
    }

    #[derive(Hash, Eq, PartialEq, Clone)]
    struct TestValue(i32);

    let mut set: IndexSet<TestValue, DummyHasher> = IndexSet {
        map: IndexMap {
            core: IndexMapCore {
                indices: Indices::new(),
                entries: Entries::new(),
            },
            hash_builder: DummyHasher,
        },
    };

    set.insert(TestValue(42));
    let (index, replaced) = set.replace_full(TestValue(43));

    let expected_replaced_value = None;
}

