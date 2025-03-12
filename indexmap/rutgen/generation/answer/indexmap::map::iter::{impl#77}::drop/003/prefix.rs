// Answer 0

#[test]
fn test_drop_with_matching_key_in_tail() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map = IndexMap::<u32, String, TestHasher> {
        core: IndexMapCore::new(),
        hash_builder: TestHasher,
    };

    let tail = {
        let mut core = IndexMapCore::with_capacity(2);
        core.entries.push(Bucket { hash: HashValue(1), key: 1, value: "old_value".to_string() });
        core.indices = Indices::new();
        core
    };

    let replace_with_vec = vec![(1, "new_value".to_string())];
    let replace_with = replace_with_vec.into_iter();

    let mut splice = Splice {
        map: &mut map,
        tail,
        drain: vec![].into_iter(),
        replace_with,
    };

    splice.drop();
}

#[test]
fn test_drop_with_multiple_replacements() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map = IndexMap::<u32, String, TestHasher> {
        core: IndexMapCore::new(),
        hash_builder: TestHasher,
    };

    let tail = {
        let mut core = IndexMapCore::with_capacity(2);
        core.entries.push(Bucket { hash: HashValue(1), key: 1, value: "old_value_1".to_string() });
        core.entries.push(Bucket { hash: HashValue(2), key: 2, value: "old_value_2".to_string() });
        core.indices = Indices::new();
        core
    };

    let replace_with_vec = vec![(1, "new_value_1".to_string()), (2, "new_value_2".to_string())];
    let replace_with = replace_with_vec.into_iter();

    let mut splice = Splice {
        map: &mut map,
        tail,
        drain: vec![].into_iter(),
        replace_with,
    };

    splice.drop();
}

#[test]
fn test_drop_with_no_existing_matches() {
    struct TestHasher;

    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map = IndexMap::<u32, String, TestHasher> {
        core: IndexMapCore::new(),
        hash_builder: TestHasher,
    };

    let tail = {
        let mut core = IndexMapCore::with_capacity(1);
        core.entries.push(Bucket { hash: HashValue(2), key: 2, value: "old_value".to_string() });
        core.indices = Indices::new();
        core
    };

    let replace_with_vec = vec![(1, "new_value".to_string())];
    let replace_with = replace_with_vec.into_iter();

    let mut splice = Splice {
        map: &mut map,
        tail,
        drain: vec![].into_iter(),
        replace_with,
    };

    splice.drop();
}

