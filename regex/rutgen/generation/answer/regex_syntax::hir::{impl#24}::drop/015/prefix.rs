// Answer 0

#[test]
fn test_drop_with_empty_capture_in_alternation() {
    let capture_expression = Capture {
        index: 0,
        name: None,
        sub: Box::new(Hir {
            kind: HirKind::Empty,
            props: Properties(Box::new(PropertiesI {})),
        }),
    };

    let repetition_expression = Repetition {
        min: 1,
        max: Some(3),
        greedy: true,
        sub: Box::new(Hir {
            kind: HirKind::Capture(capture_expression),
            props: Properties(Box::new(PropertiesI {})),
        }),
    };

    let alternation_expression = HirKind::Alternation(vec![
        Hir {
            kind: HirKind::Repetition(repetition_expression.clone()),
            props: Properties(Box::new(PropertiesI {})),
        },
        Hir {
            kind: HirKind::Repetition(repetition_expression),
            props: Properties(Box::new(PropertiesI {})),
        },
    ]);

    let mut hir = Hir {
        kind: alternation_expression,
        props: Properties(Box::new(PropertiesI {})),
    };

    drop(hir);
}

#[test]
fn test_drop_with_non_empty_sub_in_repetition() {
    let sub_expression = Hir {
        kind: HirKind::Literal(Literal { bytes: vec![b'a'] }),
        props: Properties(Box::new(PropertiesI {})),
    };

    let repetition_expression = Repetition {
        min: 0,
        max: None,
        greedy: false,
        sub: Box::new(sub_expression),
    };

    let alternation_expression = HirKind::Alternation(vec![
        Hir {
            kind: HirKind::Repetition(repetition_expression),
            props: Properties(Box::new(PropertiesI {})),
        },
    ]);

    let mut hir = Hir {
        kind: alternation_expression,
        props: Properties(Box::new(PropertiesI {})),
    };

    drop(hir);
}

