// Answer 0

#[test]
fn test_bitor_non_empty_sets_no_common_elements() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let set1: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(1, ()), (2, ())]),
    };

    let set2: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(3, ()), (4, ())]),
    };

    let _result = &set1 | &set2;
}

#[test]
fn test_bitor_non_empty_sets_all_common_elements() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let set1: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(1, ()), (2, ())]),
    };

    let set2: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(1, ()), (2, ())]),
    };

    let _result = &set1 | &set2;
}

#[test]
fn test_bitor_varied_size_sets() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let set1: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(1, ()), (2, ())]),
    };

    let set2: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(2, ()), (3, ()), (4, ())]),
    };

    let _result = &set1 | &set2;
}

#[test]
fn test_bitor_single_element_sets() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let set1: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(1, ())]),
    };

    let set2: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(2, ())]),
    };

    let _result = &set1 | &set2;
}

#[test]
fn test_bitor_empty_and_non_empty_set() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let set1: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::new(),
    };

    let set2: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(1, ()), (2, ())]),
    };

    let _result = &set1 | &set2;
}

