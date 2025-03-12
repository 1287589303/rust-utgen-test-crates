// Answer 0

#[test]
fn test_concat_with_two_hir_instances() {
    let hir1 = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };
    let hir2 = Hir {
        kind: HirKind::Char('b'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };
    let subs = vec![hir1, hir2];
    let result = Hir::concat(subs);
}

#[test]
fn test_concat_with_multiple_hir_instances() {
    let hir1 = Hir {
        kind: HirKind::Class(Class::new()),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(2),
    };
    let hir2 = Hir {
        kind: HirKind::Capture(Capture::new()),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(3),
    };
    let hir3 = Hir {
        kind: HirKind::Look(Look::new()),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };
    let subs = vec![hir1, hir2, hir3];
    let result = Hir::concat(subs);
}

#[test]
fn test_concat_with_single_empty_and_one_non_empty() {
    let hir1 = Hir {
        kind: HirKind::empty(),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    };
    let hir2 = Hir {
        kind: HirKind::Char('c'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };
    let subs = vec![hir1, hir2];
    let result = Hir::concat(subs);
}

