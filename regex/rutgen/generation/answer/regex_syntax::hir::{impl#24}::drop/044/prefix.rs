// Answer 0

#[test]
fn test_drop_repetition_with_non_empty_capture() {
    let capture = Capture {
        index: 0,
        name: Some(Box::from("test_capture")),
        sub: Box::new(Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("a")),
                    props: Properties(Box::new(PropertiesI)),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("b")),
                    props: Properties(Box::new(PropertiesI)),
                },
            ]),
            props: Properties(Box::new(PropertiesI)),
        }),
    };

    let repetition = Repetition {
        min: 1,
        max: Some(5),
        greedy: true,
        sub: Box::new(Hir {
            kind: HirKind::Capture(capture),
            props: Properties(Box::new(PropertiesI)),
        }),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties(Box::new(PropertiesI)),
    };

    drop(hir);
}

#[test]
fn test_drop_repetition_with_nested_capture() {
    let nested_capture = Capture {
        index: 1,
        name: Some(Box::from("nested_capture")),
        sub: Box::new(Hir {
            kind: HirKind::Alternation(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("x")),
                    props: Properties(Box::new(PropertiesI)),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("y")),
                    props: Properties(Box::new(PropertiesI)),
                },
            ]),
            props: Properties(Box::new(PropertiesI)),
        }),
    };

    let repetition = Repetition {
        min: 2,
        max: Some(10),
        greedy: false,
        sub: Box::new(Hir {
            kind: HirKind::Capture(nested_capture),
            props: Properties(Box::new(PropertiesI)),
        }),
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties(Box::new(PropertiesI)),
    };

    drop(hir);
}

