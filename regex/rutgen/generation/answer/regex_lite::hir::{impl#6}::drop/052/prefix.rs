// Answer 0

#[test]
fn test_drop_hir_kind_char() {
    let hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = std::mem::drop(hir);
}

#[test]
fn test_drop_hir_kind_look() {
    struct Look;
    impl Look {
        fn new() -> Self { Look }
    }

    let hir = Hir {
        kind: HirKind::Look(Look::new()),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = std::mem::drop(hir);
}

#[test]
fn test_drop_hir_kind_class() {
    struct Class;
    impl Class {
        fn new() -> Self { Class }
    }

    let hir = Hir {
        kind: HirKind::Class(Class::new()),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _ = std::mem::drop(hir);
}

#[test]
fn test_drop_hir_kind_empty() {
    let hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(0),
    };
    let _ = std::mem::drop(hir);
}

