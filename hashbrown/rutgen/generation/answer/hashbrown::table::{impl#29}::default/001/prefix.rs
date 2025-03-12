// Answer 0

#[test]
fn test_default_iter_hash() {
    let iter_hash: IterHash<u32> = IterHash::default();
}

#[test]
fn test_default_iter_hash_with_other_type() {
    let iter_hash: IterHash<String> = IterHash::default();
}

