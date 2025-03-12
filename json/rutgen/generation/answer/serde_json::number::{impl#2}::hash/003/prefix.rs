// Answer 0

#[test]
fn test_hash_with_negative_one() {
    let neg_int = N::NegInt(-1);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    neg_int.hash(&mut hasher);
}

#[test]
fn test_hash_with_negative_min() {
    let neg_int = N::NegInt(i64::MIN);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    neg_int.hash(&mut hasher);
}

#[test]
fn test_hash_with_negative_long() {
    let neg_int = N::NegInt(-9223372036854775807);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    neg_int.hash(&mut hasher);
}

