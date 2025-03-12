// Answer 0

#[test]
fn test_full_with_zero_hash() {
    let tag = Tag::full(0);
}

#[test]
fn test_full_with_maximum_hash() {
    let tag = Tag::full(0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_full_with_max_top7_bits() {
    let tag = Tag::full(0x7F);
}

#[test]
fn test_full_with_min_top7_bits() {
    let tag = Tag::full(0x80);
}

