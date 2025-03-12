// Answer 0

#[test]
fn test_subs_capture_non_empty() {
    let sub_hir = Hir {
        kind: HirKind::Literal(Literal(Box::new(b"test".to_vec()))),
        props: Properties::default(),
    };

    let capture = Capture {
        index: 0,
        name: Some(Box::from("name")),
        sub: Box::new(sub_hir),
    };

    let hir_capture = Hir {
        kind: HirKind::Capture(capture),
        props: Properties::default(),
    };

    let _subs = hir_capture.subs();
}

#[test]
fn test_subs_capture_with_another_sub() {
    let sub_hir_1 = Hir {
        kind: HirKind::Literal(Literal(Box::new(b"first".to_vec()))),
        props: Properties::default(),
    };

    let sub_hir_2 = Hir {
        kind: HirKind::Literal(Literal(Box::new(b"second".to_vec()))),
        props: Properties::default(),
    };

    let capture = Capture {
        index: 1,
        name: None,
        sub: Box::new(Hir {
            kind: HirKind::Concat(vec![sub_hir_1, sub_hir_2]),
            props: Properties::default(),
        }),
    };

    let hir_capture = Hir {
        kind: HirKind::Capture(capture),
        props: Properties::default(),
    };

    let _subs = hir_capture.subs();
}

