// Answer 0

#[test]
fn test_add_to_byteset_start_lf() {
    let mut set = crate::util::ByteClassSet::empty();
    let lineterm = DebugByte(b'\n');
    let mut matcher = LookMatcher { lineterm };

    matcher.add_to_byteset(Look::StartLF, &mut set);
}

#[test]
fn test_add_to_byteset_end_lf() {
    let mut set = crate::util::ByteClassSet::empty();
    let lineterm = DebugByte(b'\n');
    let mut matcher = LookMatcher { lineterm };

    matcher.add_to_byteset(Look::EndLF, &mut set);
}

#[test]
fn test_add_to_byteset_start_crlf() {
    let mut set = crate::util::ByteClassSet::empty();
    let lineterm = DebugByte(b'\n');
    let mut matcher = LookMatcher { lineterm };

    matcher.add_to_byteset(Look::StartCRLF, &mut set);
}

#[test]
fn test_add_to_byteset_end_crlf() {
    let mut set = crate::util::ByteClassSet::empty();
    let lineterm = DebugByte(b'\n');
    let mut matcher = LookMatcher { lineterm };

    matcher.add_to_byteset(Look::EndCRLF, &mut set);
}

#[test]
fn test_add_to_byteset_start_lf_with_empty_haystack() {
    let mut set = crate::util::ByteClassSet::empty();
    let lineterm = DebugByte(b'\n');
    let mut matcher = LookMatcher { lineterm };

    matcher.add_to_byteset(Look::StartLF, &mut set);
}

#[test]
fn test_add_to_byteset_end_lf_with_one_character_haystack() {
    let mut set = crate::util::ByteClassSet::empty();
    let lineterm = DebugByte(b'\n');
    let mut matcher = LookMatcher { lineterm };

    matcher.add_to_byteset(Look::EndLF, &mut set);
}

