// Answer 0

#[test]
fn test_add_to_byteset_word_unicode_boundary() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(b'\n');

    let look = Look::WordUnicode;
    let mut set = ByteClassSet::empty();

    matcher.add_to_byteset(look, &mut set);
}

#[test]
fn test_add_to_byteset_word_unicode_b1_max() {
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(b'\n');

    let look = Look::WordUnicode;
    let mut set = ByteClassSet::empty();

    matcher.add_to_byteset(look, &mut set);
}

