// Answer 0

#[test]
fn test_add_to_byteset_word_end_unicode_boundary() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let lineterm = DebugByte(0); // arbitrary byte for line terminator
    let matcher = LookMatcher { lineterm };
    matcher.add_to_byteset(Look::WordEndUnicode, &mut set);
}

#[test]
fn test_add_to_byteset_word_end_unicode_over_boundary() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let lineterm = DebugByte(0); // arbitrary byte for line terminator
    let matcher = LookMatcher { lineterm };
    matcher.add_to_byteset(Look::WordEndUnicode, &mut set);
}

