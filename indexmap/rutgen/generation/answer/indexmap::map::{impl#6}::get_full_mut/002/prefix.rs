// Answer 0

#[test]
fn test_get_full_mut_with_existing_key() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, String, TestHasher> = IndexMap::with_hasher(TestHasher);
    map.insert(1, "one".to_string());
    
    let key: &i32 = &1;
    let result = map.get_full_mut(key);
}

#[test]
fn test_get_full_mut_with_another_existing_key() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<String, i32, TestHasher> = IndexMap::with_hasher(TestHasher);
    map.insert("two".to_string(), 2);
    
    let key: &String = &"two".to_string();
    let result = map.get_full_mut(key);
}

#[test]
fn test_get_full_mut_on_multiple_entries() {
    struct TestHasher;
    impl BuildHasher for TestHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<i32, i32, TestHasher> = IndexMap::with_hasher(TestHasher);
    map.insert(3, 100);
    map.insert(4, 200);
    
    let key: &i32 = &3;
    let result = map.get_full_mut(key);
}

