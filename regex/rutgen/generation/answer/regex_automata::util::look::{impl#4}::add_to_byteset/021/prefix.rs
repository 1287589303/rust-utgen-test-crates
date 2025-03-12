// Answer 0

#[test]
fn test_add_to_byteset_word_end_half_ascii_beyond() {
    let mut matcher = LookMatcher::new();
    let set = &mut ByteClassSet::empty();
    matcher.add_to_byteset(Look::WordEndHalfAscii, set);
}

#[test]
fn test_add_to_byteset_word_end_half_ascii_boundary() {
    let mut matcher = LookMatcher::new();
    let set = &mut ByteClassSet::empty();
    matcher.add_to_byteset(Look::WordEndHalfAscii, set);
    matcher.add_to_byteset(Look::WordEndHalfAscii, set);
}

