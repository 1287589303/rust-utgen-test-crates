// Answer 0

#[test]
fn test_add_to_byteset_word_ascii() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(DebugByte(0));

    // Triggering matches for Look::WordAscii where b1 == 255
    matcher.add_to_byteset(Look::WordAscii, &mut set);
}

#[test]
fn test_add_to_byteset_word_ascii_b1_b2_conditions() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let mut matcher = LookMatcher::new();
    matcher.set_line_terminator(DebugByte(1));

    // Triggering matches for Look::WordAscii with the specified boundaries
    matcher.add_to_byteset(Look::WordAscii, &mut set);
}

