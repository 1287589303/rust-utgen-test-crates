// Answer 0

#[test]
fn test_add_to_byteset_word_start_ascii_b1_exceeds_limit() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let look_matcher = LookMatcher::new();
    let look = Look::WordStartAscii;

    look_matcher.add_to_byteset(look, &mut set);
}

#[test]
fn test_add_to_byteset_word_start_ascii_b1_at_limit() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let look_matcher = LookMatcher::new();
    let look = Look::WordStartAscii;

    look_matcher.add_to_byteset(look, &mut set);
}

