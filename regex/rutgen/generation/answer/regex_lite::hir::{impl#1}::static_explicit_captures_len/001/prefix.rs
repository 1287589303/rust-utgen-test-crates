// Answer 0

#[test]
fn test_static_explicit_captures_len_none() {
    let hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = hir.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_zero() {
    let hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(0),
    };
    let _ = hir.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_one() {
    let hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };
    let _ = hir.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_multiple() {
    let hir = Hir {
        kind: HirKind::Concat(vec![
            Hir::char('a'),
            Hir::char('b'),
            Hir::char('c'),
        ]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(3),
    };
    let _ = hir.static_explicit_captures_len();
}

#[test]
fn test_static_explicit_captures_len_large_value() {
    let hir = Hir {
        kind: HirKind::Capture(Capture {}),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(1000),
    };
    let _ = hir.static_explicit_captures_len();
}

