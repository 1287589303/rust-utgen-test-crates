// Answer 0

#[test]
fn test_alternation_single_element_start_anchored() {
    let single_hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };
    let subs = vec![single_hir];
    let result = Hir::alternation(subs);
}

#[test]
fn test_alternation_single_element_not_start_anchored() {
    let single_hir = Hir {
        kind: HirKind::Char('b'),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    let subs = vec![single_hir];
    let result = Hir::alternation(subs);
}

#[test]
fn test_alternation_single_element_both_conditions() {
    let single_hir = Hir {
        kind: HirKind::Char('c'),
        is_start_anchored: true,
        is_match_empty: true,
        static_explicit_captures_len: Some(2),
    };
    let subs = vec![single_hir];
    let result = Hir::alternation(subs);
}

