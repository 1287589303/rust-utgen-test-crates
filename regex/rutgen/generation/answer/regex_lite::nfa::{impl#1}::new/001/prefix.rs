// Answer 0

#[test]
fn test_nfa_new_with_none_size_limit() {
    let config = Config { size_limit: None };
    let pattern = String::from("a");
    let hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = NFA::new(config, pattern, &hir);
}

#[test]
fn test_nfa_new_with_valid_pattern_and_start_anchored() {
    let config = Config { size_limit: Some(1024) };
    let pattern = String::from("abc");
    let hir = Hir {
        kind: HirKind::Concat(vec![]),
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };

    let _ = NFA::new(config, pattern, &hir);
}

#[test]
fn test_nfa_new_with_empty_pattern() {
    let config = Config { size_limit: Some(512) };
    let pattern = String::from("");
    let hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };

    let _ = NFA::new(config, pattern, &hir);
}

#[test]
fn test_nfa_new_with_capture_groups() {
    let config = Config { size_limit: Some(256) };
    let pattern = String::from("a(bc)?");
    let hir = Hir {
        kind: HirKind::Repetition(Box::new(hir::Repetition {
            min: 0,
            max: 1,
            greedy: true,
        })),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };

    let _ = NFA::new(config, pattern, &hir);
}

#[test]
fn test_nfa_new_with_max_size_limit() {
    let config = Config { size_limit: Some(usize::MAX) };
    let pattern = String::from(".*");
    let hir = Hir {
        kind: HirKind::Class(hir::Class::Any),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = NFA::new(config, pattern, &hir);
}

#[test]
fn test_nfa_new_with_mixed_captures() {
    let config = Config { size_limit: Some(128) };
    let pattern = String::from("a(b)c");
    let hir = Hir {
        kind: HirKind::Concat(vec![
            Box::new(hir::Hir { kind: HirKind::Char('a'), ..Default::default() }),
            Box::new(hir::Hir { kind: HirKind::Capture(0, Box::new(hir::Hir { kind: HirKind::Char('b'), ..Default::default() })), ..Default::default() }),
            Box::new(hir::Hir { kind: HirKind::Char('c'), ..Default::default() }),
        ]),
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };

    let _ = NFA::new(config, pattern, &hir);
}

