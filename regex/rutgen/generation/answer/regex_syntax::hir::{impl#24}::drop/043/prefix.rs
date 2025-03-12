// Answer 0

#[test]
fn test_drop_repetition_with_concat() {
    let sub_hir_concat = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::new("a")),
                props: Properties(Box::new(PropertiesI::default())),
            },
            Hir {
                kind: HirKind::Literal(Literal::new("b")),
                props: Properties(Box::new(PropertiesI::default())),
            },
        ]),
        props: Properties(Box::new(PropertiesI::default())),
    };

    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            min: 1,
            max: Some(5),
            greedy: true,
            sub: Box::new(sub_hir_concat),
        }),
        props: Properties(Box::new(PropertiesI::default())),
    };

    let _ = repetition_hir; // This would invoke the drop method of Hir
}

#[test]
fn test_drop_repetition_with_empty_concat() {
    let sub_hir_concat = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::new("x")),
                props: Properties(Box::new(PropertiesI::default())),
            },
            Hir {
                kind: HirKind::Literal(Literal::new("y")),
                props: Properties(Box::new(PropertiesI::default())),
            },
        ]),
        props: Properties(Box::new(PropertiesI::default())),
    };

    let repetition_hir = Hir {
        kind: HirKind::Repetition(Repetition {
            min: 2,
            max: Some(4),
            greedy: false,
            sub: Box::new(sub_hir_concat),
        }),
        props: Properties(Box::new(PropertiesI::default())),
    };

    let _ = repetition_hir; // This would invoke the drop method of Hir
}

