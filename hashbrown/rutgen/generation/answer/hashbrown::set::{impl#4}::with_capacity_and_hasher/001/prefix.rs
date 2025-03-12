// Answer 0

#[test]
fn test_with_capacity_and_hasher_zero_capacity() {
    let hasher = DefaultHashBuilder::default();
    let set = HashSet::with_capacity_and_hasher(0, hasher);
}

#[test]
fn test_with_capacity_and_hasher_capacity_one() {
    let hasher = DefaultHashBuilder::default();
    let set = HashSet::with_capacity_and_hasher(1, hasher);
}

#[test]
fn test_with_capacity_and_hasher_capacity_five() {
    let hasher = DefaultHashBuilder::default();
    let set = HashSet::with_capacity_and_hasher(5, hasher);
}

#[test]
fn test_with_capacity_and_hasher_capacity_ten() {
    let hasher = DefaultHashBuilder::default();
    let set = HashSet::with_capacity_and_hasher(10, hasher);
}

#[test]
fn test_with_capacity_and_hasher_custom_hasher() {
    struct CustomHasher;
    impl BuildHasher for CustomHasher {
        type Hasher = std::collections::hash_map::RandomState;

        fn build_hasher(&self) -> Self::Hasher {
            std::collections::hash_map::RandomState::new()
        }
    }
    let hasher = CustomHasher;
    let set = HashSet::with_capacity_and_hasher(10, hasher);
}

