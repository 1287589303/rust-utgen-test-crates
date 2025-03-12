// Answer 0

#[test]
fn test_insert_existing_key_with_string_key() {
    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<String, i32, CustomHasher> = IndexMap::default();
    map.insert("key1".to_string(), 10);
    let result = map.insert("key1".to_string(), 20);
    // result should contain Some(10)
}

#[test]
fn test_insert_new_key_with_string_key() {
    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<String, i32, CustomHasher> = IndexMap::default();
    let result = map.insert("key2".to_string(), 30);
    // result should be None
}

#[test]
fn test_insert_existing_key_with_integer_key() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = IndexMap::default();
    map.insert(1, 100);
    let result = map.insert(1, 200);
    // result should contain Some(100)
}

#[test]
fn test_insert_new_key_with_integer_key() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = IndexMap::default();
    let result = map.insert(2, 300);
    // result should be None
}

#[test]
fn test_insert_boundary_keys_with_string_key() {
    let mut map: IndexMap<String, i32, std::collections::hash_map::RandomState> = IndexMap::default();
    let result_min = map.insert("a".to_string(), 1);
    let result_max = map.insert("z".to_string(), 26);
    // result_min should be None
    // result_max should be None
}

#[test]
fn test_insert_boundary_keys_with_integer_key() {
    let mut map: IndexMap<i32, i32, std::collections::hash_map::RandomState> = IndexMap::default();
    let result_min = map.insert(i32::MIN, 0);
    let result_max = map.insert(i32::MAX, 100);
    // result_min should be None
    // result_max should be None
}

#[test]
fn test_insert_with_custom_hasher() {
    struct CustomHasher;

    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::DefaultHasher;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::DefaultHasher::new()
        }
    }

    let mut map: IndexMap<String, i32, CustomHasher> = IndexMap::default();
    let result1 = map.insert("key3".to_string(), 50);
    let result2 = map.insert("key4".to_string(), 60);
    // result1 should be None
    // result2 should be None
}

