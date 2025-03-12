// Answer 0

#[test]
fn test_insert_full_new_value() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
    }

    let mut index_set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher },
    };

    let (index, existing) = index_set.insert_full(10);
}

#[test]
fn test_insert_full_existing_value() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
    }

    let mut index_set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher },
    };

    index_set.insert_full(20);
    let (index, existing) = index_set.insert_full(20);
}

#[test]
fn test_insert_full_multiple_values() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
    }

    let mut index_set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher },
    };

    index_set.insert_full(30);
    index_set.insert_full(40);
    let (index_existing_value, existing) = index_set.insert_full(30);
    let (index_new_value, new_existing) = index_set.insert_full(50);
}

#[test]
fn test_insert_full_boundary_case() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;
    }

    let mut index_set: super::IndexSet<i32, TestHasher> = super::IndexSet {
        map: super::IndexMap { core: super::IndexMapCore::new(), hash_builder: TestHasher },
    };

    let (index_zero, existing_zero) = index_set.insert_full(0);
    let (index_neg_one, existing_neg_one) = index_set.insert_full(-1);
}

