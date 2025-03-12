// Answer 0

#[test]
fn test_concat_multiple_hirs_with_no_captures() {
    let hir1 = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let hir2 = Hir {
        kind: HirKind::Char('b'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let result = Hir::concat(vec![hir1, hir2]);
}

#[test]
fn test_concat_multiple_hirs_with_some_captures() {
    let hir1 = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(2),
    };
    let hir2 = Hir {
        kind: HirKind::Char('b'),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(1),
    };
    let result = Hir::concat(vec![hir1, hir2]);
}

#[test]
fn test_concat_multiple_hirs_with_all_empty() {
    let hir1 = Hir {
        kind: HirKind::Empty,
        is_start_anchored: true,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    };
    let hir2 = Hir {
        kind: HirKind::Empty,
        is_start_anchored: true,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    };
    let result = Hir::concat(vec![hir1, hir2]);
}

#[test]
fn test_concat_multiple_hirs_with_mixed_match_empty() {
    let hir1 = Hir {
        kind: HirKind::Char('c'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let hir2 = Hir {
        kind: HirKind::Char('d'),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(3),
    };
    let result = Hir::concat(vec![hir1, hir2]);
}

