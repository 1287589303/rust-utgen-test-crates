// Answer 0

#[test]
fn test_subs_char() {
    let hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = hir.kind.subs();
}

#[test]
fn test_subs_empty() {
    let hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = hir.kind.subs();
}

#[test]
fn test_subs_look() {
    let hir = Hir {
        kind: HirKind::Look(Look::Start),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = hir.kind.subs();
}

#[test]
fn test_subs_class() {
    let hir = Hir {
        kind: HirKind::Class(Class {
            ranges: vec![],
        }),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = hir.kind.subs();
}

