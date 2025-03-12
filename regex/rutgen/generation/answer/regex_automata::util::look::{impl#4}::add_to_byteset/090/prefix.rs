// Answer 0

#[test]
fn test_add_to_byteset_start() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let lineterm = DebugByte(0);
    let matcher = LookMatcher { lineterm };

    matcher.add_to_byteset(Look::Start, &mut set);
}

#[test]
fn test_add_to_byteset_end() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let lineterm = DebugByte(255);
    let matcher = LookMatcher { lineterm };

    matcher.add_to_byteset(Look::End, &mut set);
}

#[test]
fn test_add_to_byteset_start_lineterm() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let lineterm = DebugByte(128);
    let matcher = LookMatcher { lineterm };

    matcher.add_to_byteset(Look::Start, &mut set);
}

#[test]
fn test_add_to_byteset_end_lineterm() {
    let mut set = crate::util::alphabet::ByteClassSet::empty();
    let lineterm = DebugByte(64);
    let matcher = LookMatcher { lineterm };

    matcher.add_to_byteset(Look::End, &mut set);
}

