// Answer 0

#[test]
fn test_indexmap_default_with_default_hash_builder() {
    struct DefaultHasher;

    impl Default for DefaultHasher {
        fn default() -> Self {
            DefaultHasher
        }
    }

    let map: crate::IndexMap<i32, i32, DefaultHasher> = crate::IndexMap::default();
}

#[test]
fn test_indexmap_default_with_random_state() {
    #[cfg(feature = "std")]
    let map: crate::IndexMap<i32, i32, std::collections::hash_map::RandomState> = crate::IndexMap::default();
}

