// Answer 0

#[test]
fn test_add_to_byteset_word_start_half_unicode_b1_over_limit() {
    let look = Look::WordStartHalfUnicode;
    let mut set = ByteClassSet::empty();
    let matcher = LookMatcher::new();

    matcher.add_to_byteset(look, &mut set);
}

#[test]
#[should_panic]
fn test_add_to_byteset_word_start_half_unicode_b1_equals_limit() {
    let look = Look::WordStartHalfUnicode;
    let mut set = ByteClassSet::empty();
    let matcher = LookMatcher::new();

    for _ in 0..256 {
        matcher.add_to_byteset(look, &mut set);
    }
}

