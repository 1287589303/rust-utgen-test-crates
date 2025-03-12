// Answer 0

#[test]
fn test_subs_alternation_with_single_capture() {
    let capture = Capture {
        index: 0,
        name: None,
        sub: Box::new(Hir {
            kind: HirKind::Empty,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        }),
    };

    let alternation = HirKind::Alternation(vec![Hir {
        kind: HirKind::Capture(capture),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    }]);
    
    let hir = Hir {
        kind: alternation,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = hir.kind.subs();
}

#[test]
fn test_subs_alternation_with_multiple_repetitions() {
    let repetition1 = Repetition {
        min: 1,
        max: Some(3),
        greedy: true,
        sub: Box::new(Hir {
            kind: HirKind::Char('a'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        }),
    };
    
    let repetition2 = Repetition {
        min: 0,
        max: None,
        greedy: false,
        sub: Box::new(Hir {
            kind: HirKind::Char('b'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        }),
    };

    let alternation = HirKind::Alternation(vec![
        Hir { kind: HirKind::Repetition(repetition1), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None },
        Hir { kind: HirKind::Repetition(repetition2), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None },
    ]);
    
    let hir = Hir {
        kind: alternation,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = hir.kind.subs();
}

#[test]
fn test_subs_alternation_with_concat() {
    let concat = HirKind::Concat(vec![
        Hir {
            kind: HirKind::Char('a'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        },
        Hir {
            kind: HirKind::Char('b'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        },
    ]);

    let alternation = HirKind::Alternation(vec![
        Hir { kind: concat, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None },
    ]);
    
    let hir = Hir {
        kind: alternation,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = hir.kind.subs();
}

#[test]
fn test_subs_alternation_with_mixed_expressions() {
    let capture = Capture {
        index: 1,
        name: Some(Box::from("captured")),
        sub: Box::new(Hir {
            kind: HirKind::Empty,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        }),
    };

    let repetition = Repetition {
        min: 2,
        max: Some(5),
        greedy: true,
        sub: Box::new(Hir {
            kind: HirKind::Char('c'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        }),
    };

    let alternation = HirKind::Alternation(vec![
        Hir { kind: HirKind::Capture(capture), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None },
        Hir { kind: HirKind::Repetition(repetition), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None },
    ]);
    
    let hir = Hir {
        kind: alternation,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = hir.kind.subs();
}

