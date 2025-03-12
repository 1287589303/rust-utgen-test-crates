// Answer 0

#[test]
fn test_hash_posint_min() {
    let value = N::PosInt(1);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
}

#[test]
fn test_hash_posint_mid() {
    let value = N::PosInt(123456);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
}

#[test]
fn test_hash_posint_max() {
    let value = N::PosInt(u64::MAX);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
}

