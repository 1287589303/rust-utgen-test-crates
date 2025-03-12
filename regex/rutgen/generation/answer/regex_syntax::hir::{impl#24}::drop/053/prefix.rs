// Answer 0

#[test]
fn test_drop_empty() {
    let hir = Hir {
        kind: HirKind::Empty,
        props: Properties(Box::new(PropertiesI)),
    };
    // This will call drop on an HirKind::Empty
    drop(hir);
}

#[test]
fn test_drop_literal() {
    let hir = Hir {
        kind: HirKind::Literal(Literal::from("test")),
        props: Properties(Box::new(PropertiesI)),
    };
    // This will call drop on an HirKind::Literal
    drop(hir);
}

#[test]
fn test_drop_class() {
    let hir = Hir {
        kind: HirKind::Class(Class::new(vec!['a', 'b', 'c'])),
        props: Properties(Box::new(PropertiesI)),
    };
    // This will call drop on an HirKind::Class
    drop(hir);
}

#[test]
fn test_drop_look() {
    let hir = Hir {
        kind: HirKind::Look(Look::new()),
        props: Properties(Box::new(PropertiesI)),
    };
    // This will call drop on an HirKind::Look
    drop(hir);
}

#[test]
fn test_drop_capture_empty_sub() {
    let capture = Capture {
        index: 0,
        name: None,
        sub: Box::new(Hir {
            kind: HirKind::Empty,
            props: Properties(Box::new(PropertiesI)),
        }),
    };
    let hir = Hir {
        kind: HirKind::Capture(capture),
        props: Properties(Box::new(PropertiesI)),
    };
    // This will call drop on an HirKind::Capture with an empty sub
    drop(hir);
}

#[test]
fn test_drop_repetition_empty_sub() {
    let repetition = Repetition {
        min: 0,
        max: None,
        greedy: true,
        sub: Box::new(Hir {
            kind: HirKind::Empty,
            props: Properties(Box::new(PropertiesI)),
        }),
    };
    let hir = Hir {
        kind: HirKind::Repetition(repetition),
        props: Properties(Box::new(PropertiesI)),
    };
    // This will call drop on an HirKind::Repetition with an empty sub
    drop(hir);
}

#[test]
fn test_drop_concat_empty() {
    let hir = Hir {
        kind: HirKind::Concat(vec![]),
        props: Properties(Box::new(PropertiesI)),
    };
    // This will call drop on an HirKind::Concat that is empty
    drop(hir);
}

#[test]
fn test_drop_alternation_empty() {
    let hir = Hir {
        kind: HirKind::Alternation(vec![]),
        props: Properties(Box::new(PropertiesI)),
    };
    // This will call drop on an HirKind::Alternation that is empty
    drop(hir);
}

