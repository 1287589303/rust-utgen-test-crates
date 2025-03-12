// Answer 0

#[test]
fn test_is_reverse_true() {
    let nfa = NFA(Arc::new(Inner {
        reverse: true,
        ..Default::default()
    }));
    let _ = nfa.is_reverse();
}

#[test]
fn test_is_reverse_false() {
    let nfa = NFA(Arc::new(Inner {
        reverse: false,
        ..Default::default()
    }));
    let _ = nfa.is_reverse();
}

