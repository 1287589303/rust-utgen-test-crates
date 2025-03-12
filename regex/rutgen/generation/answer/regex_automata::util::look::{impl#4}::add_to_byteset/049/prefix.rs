// Answer 0

#[test]
fn test_add_to_byteset_word_end_ascii_beyond_limit() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let look = Look::WordEndAscii;
    let lineterm = DebugByte(10);
    let matcher = LookMatcher {
        lineterm,
    };
    let haystack: &[u8] = b"Some valid ASCII text.";
    let at = 24;
    matcher.add_to_byteset(look, &mut set);
}

#[test]
fn test_add_to_byteset_word_end_ascii_beyond_limit_max() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let look = Look::WordEndAscii;
    let lineterm = DebugByte(10);
    let matcher = LookMatcher {
        lineterm,
    };
    let haystack: &[u8] = b"Another ASCII text!";
    let at = 20;
    matcher.add_to_byteset(look, &mut set);
}

