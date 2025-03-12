// Answer 0

#[test]
fn test_insert_distinct_elements() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::default(),
            hash_builder: TestHasher,
        },
    };

    // Insert distinct elements
    let _ = set.insert(1);
    let _ = set.insert(2);
    let _ = set.insert(3);
    let _ = set.insert(4);
}

#[test]
fn test_insert_existing_element() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::default(),
            hash_builder: TestHasher,
        },
    };

    let _ = set.insert(1);
    let _ = set.insert(1); // Attempt to insert duplicate
}

#[test]
fn test_insert_null_value() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<Option<i32>, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::default(),
            hash_builder: TestHasher,
        },
    };

    let _ = set.insert(Some(1));
    let _ = set.insert(None); // Insert None as a valid value
}

#[test]
fn test_insert_boundary_values() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap {
            core: super::IndexMapCore::default(),
            hash_builder: TestHasher,
        },
    };

    let _ = set.insert(i32::MIN);
    let _ = set.insert(i32::MAX);
}

