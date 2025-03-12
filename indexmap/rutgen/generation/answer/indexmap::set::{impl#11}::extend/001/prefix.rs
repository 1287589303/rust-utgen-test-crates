// Answer 0

#[test]
fn test_extend_empty_iterator() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: IndexSet<i32, TestHasher> = IndexSet { map: IndexMap { core: IndexMapCore { /* initialize as needed */ }, hash_builder: TestHasher } };
    let empty_iter: Vec<i32> = vec![];
    index_set.extend(empty_iter.iter());
}

#[test]
fn test_extend_single_item_iterator() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: IndexSet<i32, TestHasher> = IndexSet { map: IndexMap { core: IndexMapCore { /* initialize as needed */ }, hash_builder: TestHasher } };
    let single_item_iter = vec![42];
    index_set.extend(single_item_iter.iter());
}

#[test]
fn test_extend_multiple_items_iterator() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: IndexSet<i32, TestHasher> = IndexSet { map: IndexMap { core: IndexMapCore { /* initialize as needed */ }, hash_builder: TestHasher } };
    let multiple_items_iter = vec![1, 2, 3, 4, 5];
    index_set.extend(multiple_items_iter.iter());
}

#[test]
fn test_extend_boundary_values_iterator() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }

    let mut index_set: IndexSet<i32, TestHasher> = IndexSet { map: IndexMap { core: IndexMapCore { /* initialize as needed */ }, hash_builder: TestHasher } };
    let boundary_values_iter = vec![i32::MIN, -1, 0, 1, i32::MAX];
    index_set.extend(boundary_values_iter.iter());
}

