// Answer 0

#[test]
fn test_subs_concat_with_repetition() {
    let repetition = Repetition {
        min: 1,
        max: Some(2),
        greedy: true,
        sub: Box::new(Hir {
            kind: HirKind::Char('a'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        }),
    };

    let hir_concat = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Repetition(repetition),
                is_start_anchored: false,
                is_match_empty: false,
                static_explicit_captures_len: None,
            },
        ]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = hir_concat.kind.subs();
}

#[test]
fn test_subs_concat_with_capture() {
    let capture = Capture {
        index: 0,
        name: Some(Box::from("group")),
        sub: Box::new(Hir {
            kind: HirKind::Char('b'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        }),
    };

    let hir_concat = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Capture(capture),
                is_start_anchored: false,
                is_match_empty: false,
                static_explicit_captures_len: None,
            },
        ]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = hir_concat.kind.subs();
}

#[test]
fn test_subs_concat_with_nested_concat() {
    let nested_concat = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Char('c'),
                is_start_anchored: false,
                is_match_empty: false,
                static_explicit_captures_len: None,
            },
            Hir {
                kind: HirKind::Char('d'),
                is_start_anchored: false,
                is_match_empty: false,
                static_explicit_captures_len: None,
            },
        ]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let hir_concat = Hir {
        kind: HirKind::Concat(vec![
            nested_concat,
        ]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _ = hir_concat.kind.subs();
}

