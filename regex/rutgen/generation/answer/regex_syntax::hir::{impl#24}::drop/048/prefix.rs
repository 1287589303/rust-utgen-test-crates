// Answer 0

#[test]
fn test_drop_with_repetition_and_literal() {
    let sub_expression = Box::new(Hir {
        kind: HirKind::Literal(Literal::from("test")),
        props: Properties(Box::new(PropertiesI::new())),
    });

    let repetition = Repetition {
        min: 1,
        max: Some(5),
        greedy: true,
        sub: sub_expression,
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties(Box::new(PropertiesI::new())),
    };

    let _ = hir; // Function under test
}

#[test]
fn test_drop_with_repetition_and_class() {
    let sub_expression = Box::new(Hir {
        kind: HirKind::Class(Class::new(vec!['a', 'b', 'c'])),
        props: Properties(Box::new(PropertiesI::new())),
    });

    let repetition = Repetition {
        min: 2,
        max: None,
        greedy: false,
        sub: sub_expression,
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties(Box::new(PropertiesI::new())),
    };

    let _ = hir; // Function under test
}

#[test]
fn test_drop_with_repetition_and_look_around() {
    let sub_expression = Box::new(Hir {
        kind: HirKind::Look(Look::new()),
        props: Properties(Box::new(PropertiesI::new())),
    });

    let repetition = Repetition {
        min: 0,
        max: Some(10),
        greedy: true,
        sub: sub_expression,
    };

    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties(Box::new(PropertiesI::new())),
    };

    let _ = hir; // Function under test
}

