// Answer 0

#[test]
fn test_bitand_non_empty_common_elements() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let set1: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(1, ()), (2, ()), (3, ())]),
    };
    let set2: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(2, ()), (3, ()), (4, ())]),
    };

    let intersection = &set1 & &set2;
}

#[test]
fn test_bitand_one_set_empty() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let set1: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(1, ()), (2, ()), (3, ())]),
    };
    let set2: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::new(),
    };

    let intersection = &set1 & &set2;
}

#[test]
fn test_bitand_both_sets_empty() {
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
        map: super::IndexMap::new(),
    };

    let intersection = &set1 & &set2;
}

#[test]
fn test_bitand_no_common_elements() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let set1: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(1, ()), (2, ()), (3, ())]),
    };
    let set2: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap::from_iter(vec![(4, ()), (5, ()), (6, ())]),
    };

    let intersection = &set1 & &set2;
}

