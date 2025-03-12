// Answer 0

#[test]
fn test_alternation_multiple_elements_all_start_anchored() {
    let subs = vec![
        Hir {
            kind: HirKind::Char('a'),
            is_start_anchored: true,
            is_match_empty: false,
            static_explicit_captures_len: Some(1),
        },
        Hir {
            kind: HirKind::Char('b'),
            is_start_anchored: true,
            is_match_empty: false,
            static_explicit_captures_len: Some(1),
        }
    ];
    let _result = Hir::alternation(subs);
}

#[test]
fn test_alternation_multiple_elements_mixed_anchored() {
    let subs = vec![
        Hir {
            kind: HirKind::Char('x'),
            is_start_anchored: true,
            is_match_empty: false,
            static_explicit_captures_len: Some(1),
        },
        Hir {
            kind: HirKind::Char('y'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: Some(1),
        }
    ];
    let _result = Hir::alternation(subs);
}

#[test]
fn test_alternation_multiple_elements_all_empty_matches() {
    let subs = vec![
        Hir {
            kind: HirKind::Empty,
            is_start_anchored: false,
            is_match_empty: true,
            static_explicit_captures_len: None,
        },
        Hir {
            kind: HirKind::Empty,
            is_start_anchored: false,
            is_match_empty: true,
            static_explicit_captures_len: None,
        }
    ];
    let _result = Hir::alternation(subs);
}

#[test]
fn test_alternation_multiple_elements_varied_captures() {
    let subs = vec![
        Hir {
            kind: HirKind::Char('m'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: Some(2),
        },
        Hir {
            kind: HirKind::Char('n'),
            is_start_anchored: false,
            is_match_empty: true,
            static_explicit_captures_len: Some(3),
        }
    ];
    let _result = Hir::alternation(subs);
}

#[test]
fn test_alternation_multiple_elements_mixed_empty_and_captures() {
    let subs = vec![
        Hir {
            kind: HirKind::Empty,
            is_start_anchored: true,
            is_match_empty: true,
            static_explicit_captures_len: None,
        },
        Hir {
            kind: HirKind::Char('z'),
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: Some(1),
        }
    ];
    let _result = Hir::alternation(subs);
}

