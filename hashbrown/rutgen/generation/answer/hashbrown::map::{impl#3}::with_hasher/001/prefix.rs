// Answer 0

#[test]
fn test_with_hasher_default_hash_builder() {
    let s = DefaultHashBuilder::default();
    let map: HashMap<i32, i32, DefaultHashBuilder> = HashMap::with_hasher(s);
}

#[test]
fn test_with_hasher_custom_build_hasher() {
    struct CustomHasher;
    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::RandomState;
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }
    let s = CustomHasher;
    let map: HashMap<String, String, CustomHasher> = HashMap::with_hasher(s);
}

#[test]
fn test_with_hasher_empty_capacity() {
    let s = DefaultHashBuilder::default();
    let map: HashMap<u32, u32> = HashMap::with_hasher(s);
}

