// Answer 0

#[test]
fn test_is_utf8_enabled() {
    let nfa = NFA(Arc::new(Inner {
        utf8: true,
        ..Default::default()
    }));
    let _ = nfa.is_utf8();
}

#[test]
fn test_is_utf8_disabled() {
    let nfa = NFA(Arc::new(Inner {
        utf8: false,
        ..Default::default()
    }));
    let _ = nfa.is_utf8();
}

