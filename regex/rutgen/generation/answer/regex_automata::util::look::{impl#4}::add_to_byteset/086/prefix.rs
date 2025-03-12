// Answer 0

#[test]
fn test_add_to_byteset_start_crlf() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let matcher = LookMatcher::new();
    matcher.set_line_terminator(b'\n');
    matcher.add_to_byteset(Look::StartCRLF, &mut set);
}

#[test]
fn test_add_to_byteset_end_crlf() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let matcher = LookMatcher::new();
    matcher.set_line_terminator(b'\n');
    matcher.add_to_byteset(Look::EndCRLF, &mut set);
}

#[test]
fn test_add_to_byteset_multiple_bytes_start_crlf() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let matcher = LookMatcher::new();
    matcher.set_line_terminator(0);
    matcher.add_to_byteset(Look::StartCRLF, &mut set);
}

#[test]
fn test_add_to_byteset_multiple_bytes_end_crlf() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let matcher = LookMatcher::new();
    matcher.set_line_terminator(255);
    matcher.add_to_byteset(Look::EndCRLF, &mut set);
}

