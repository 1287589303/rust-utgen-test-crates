// Answer 0

#[test]
fn test_extract_capture_with_prefix() {
    struct TestHir {
        kind: HirKind,
    }

    let extractor = Extractor::new().kind(ExtractKind::Prefix);
    let sub_hir = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal {
                    bytes: vec![b'a'],
                    exact: true,
                }),
            },
            Hir {
                kind: HirKind::Repetition(hir::Repetition {
                    min: 1,
                    max: Some(2),
                    greedy: true,
                    sub: Box::new(Hir {
                        kind: HirKind::Literal(Literal {
                            bytes: vec![b'b'],
                            exact: true,
                        }),
                    }),
                }),
            },
        ]),
        props: Default::default(),
    };
    let hir = Hir {
        kind: HirKind::Capture { sub: Box::new(sub_hir) },
        props: Default::default(),
    };

    extractor.extract(&hir);
}

#[test]
fn test_extract_capture_with_suffix() {
    struct TestHir {
        kind: HirKind,
    }

    let extractor = Extractor::new().kind(ExtractKind::Suffix);
    let sub_hir = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Repetition(hir::Repetition {
                    min: 1,
                    max: Some(3),
                    greedy: false,
                    sub: Box::new(Hir {
                        kind: HirKind::Literal(Literal {
                            bytes: vec![b'x'],
                            exact: true,
                        }),
                    }),
                }),
            },
            Hir {
                kind: HirKind::Literal(Literal {
                    bytes: vec![b'y'],
                    exact: true,
                }),
            },
        ]),
        props: Default::default(),
    };
    let hir = Hir {
        kind: HirKind::Capture { sub: Box::new(sub_hir) },
        props: Default::default(),
    };

    extractor.extract(&hir);
}

