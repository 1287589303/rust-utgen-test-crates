// Answer 0

#[test]
fn test_concat_empty_hir() {
    let subs: Vec<Hir> = vec![];
    let result = Hir::concat(subs);
}

#[test]
fn test_concat_single_hir() {
    let single_hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };
    let subs = vec![single_hir];
    let result = Hir::concat(subs);
}

#[test]
fn test_concat_multiple_hir_all_non_empty() {
    let hir1 = Hir {
        kind: HirKind::Char('b'),
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(2),
    };
    let hir2 = Hir {
        kind: HirKind::Char('c'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(3),
    };
    let subs = vec![hir1.clone(), hir2.clone()];
    let result = Hir::concat(subs);
}

#[test]
fn test_concat_multiple_hir_mixed_states() {
    let hir1 = Hir {
        kind: HirKind::Char('d'),
        is_start_anchored: true,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    };
    let hir2 = Hir {
        kind: HirKind::Char('e'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(2),
    };
    let subs = vec![hir1.clone(), hir2.clone()];
    let result = Hir::concat(subs);
}

#[test]
fn test_concat_multiple_hir_with_none_captures() {
    let hir1 = Hir {
        kind: HirKind::Char('f'),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    let hir2 = Hir {
        kind: HirKind::Class(Class::Any),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };
    let subs = vec![hir1.clone(), hir2.clone()];
    let result = Hir::concat(subs);
}

