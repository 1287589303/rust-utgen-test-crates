// Answer 0

#[test]
fn test_fmt_with_empty_states() {
    use crate::nfa::thompson::nfa::Inner;
    use core::fmt::Formatter;

    let formatter = &mut Formatter::new();
    let nfa = Inner {
        states: vec![],
        start_anchored: StateID(SmallIndex::from_usize(usize::MAX)),
        start_unanchored: StateID(SmallIndex::from_usize(usize::MAX)),
        start_pattern: vec![StateID(SmallIndex::from_usize(0))],
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };

    let _ = nfa.fmt(formatter);
}

#[test]
fn test_fmt_with_single_pattern() {
    use crate::nfa::thompson::nfa::Inner;
    use core::fmt::Formatter;

    let formatter = &mut Formatter::new();
    let nfa = Inner {
        states: vec![],
        start_anchored: StateID(SmallIndex::from_usize(usize::MAX)),
        start_unanchored: StateID(SmallIndex::from_usize(usize::MAX)),
        start_pattern: vec![StateID(SmallIndex::from_usize(0))],
        byte_classes: ByteClasses([0; 256]),
        ..Default::default()
    };

    let _ = nfa.fmt(formatter);
}

#[test]
fn test_fmt_with_invalid_byte_classes() {
    use crate::nfa::thompson::nfa::Inner;
    use core::fmt::Formatter;

    let formatter = &mut Formatter::new();
    let nfa = Inner {
        states: vec![],
        start_anchored: StateID(SmallIndex::from_usize(usize::MAX)),
        start_unanchored: StateID(SmallIndex::from_usize(usize::MAX)),
        start_pattern: vec![StateID(SmallIndex::from_usize(0))],
        byte_classes: ByteClasses([0; 256]), // In an invalid state for testing
        ..Default::default()
    };

    let _ = nfa.fmt(formatter);
}

