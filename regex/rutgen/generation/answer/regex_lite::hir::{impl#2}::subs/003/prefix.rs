// Answer 0

#[test]
fn test_subs_capture_with_non_empty_sub() {
    let hir_sub = Box::new(Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    });
    let capture = Capture {
        index: 1,
        name: None,
        sub: hir_sub,
    };
    let hir_kind = HirKind::Capture(capture);
    let hir = Hir {
        kind: hir_kind,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _result = hir.kind.subs();
}

#[test]
fn test_subs_capture_with_different_sub() {
    let hir_sub = Box::new(Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Char('b'),
                is_start_anchored: true,
                is_match_empty: false,
                static_explicit_captures_len: None,
            },
            Hir {
                kind: HirKind::Class(Class {
                    ranges: vec![],
                }),
                is_start_anchored: false,
                is_match_empty: false,
                static_explicit_captures_len: None,
            },
        ]),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    });
    let capture = Capture {
        index: 2,
        name: Some(Box::from("name")),
        sub: hir_sub,
    };
    let hir_kind = HirKind::Capture(capture);
    let hir = Hir {
        kind: hir_kind,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let _result = hir.kind.subs();
}

