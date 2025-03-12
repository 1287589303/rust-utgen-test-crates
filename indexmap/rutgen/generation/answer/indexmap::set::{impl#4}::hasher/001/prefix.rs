// Answer 0

#[test]
fn test_hasher_with_random_state() {
    use std::collections::hash_map::RandomState;

    let hash_builder = RandomState::new();
    let index_set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(10, hash_builder);
    let _hasher = index_set.hasher();
}

#[test]
fn test_hasher_with_custom_hasher() {
    use std::hash::BuildHasher;
    
    struct CustomHasher;
    
    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::RandomState;
        
        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }
    
    let hash_builder = CustomHasher;
    let index_set: IndexSet<i32, CustomHasher> = IndexSet::with_capacity_and_hasher(10, hash_builder);
    let _hasher = index_set.hasher();
}

#[test]
fn test_hasher_with_empty_set() {
    use std::collections::hash_map::RandomState;

    let hash_builder = RandomState::new();
    let index_set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(0, hash_builder);
    let _hasher = index_set.hasher();
}

#[test]
fn test_hasher_with_large_capacity() {
    use std::collections::hash_map::RandomState;

    let hash_builder = RandomState::new();
    let index_set: IndexSet<i32, RandomState> = IndexSet::with_capacity_and_hasher(1000, hash_builder);
    let _hasher = index_set.hasher();
}

