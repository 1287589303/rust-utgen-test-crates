// Answer 0

#[test]
fn test_drop_repetition_with_non_empty_sub_and_stack_pop_alternation() {
    use crate::{hir::Hir, hir::HirKind, hir::Repetition};

    let sub_expression = Hir {
        kind: HirKind::Alternation(vec![
            Hir {
                kind: HirKind::Literal(Default::default()),
                props: Properties(Box::new(Default::default())),
            },
            Hir {
                kind: HirKind::Literal(Default::default()),
                props: Properties(Box::new(Default::default())),
            },
        ]),
        props: Properties(Box::new(Default::default())),
    };

    let repetition_expression = Repetition {
        min: 1,
        max: Some(5),
        greedy: true,
        sub: Box::new(sub_expression),
    };

    let hir_instance = Hir {
        kind: HirKind::Repetition(repetition_expression),
        props: Properties(Box::new(Default::default())),
    };

    let _ = hir_instance; // This will invoke the drop method when _ goes out of scope
}

#[test]
fn test_drop_repetition_with_alternation_sub() {
    use crate::{hir::Hir, hir::HirKind, hir::Repetition};

    let alternation_sub = Hir {
        kind: HirKind::Alternation(vec![
            Hir {
                kind: HirKind::Literal(Default::default()),
                props: Properties(Box::new(Default::default())),
            },
            Hir {
                kind: HirKind::Class(Default::default()),
                props: Properties(Box::new(Default::default())),
            },
        ]),
        props: Properties(Box::new(Default::default())),
    };

    let repetition_instance = Repetition {
        min: 2,
        max: None,
        greedy: false,
        sub: Box::new(alternation_sub),
    };

    let hir_instance = Hir {
        kind: HirKind::Repetition(repetition_instance),
        props: Properties(Box::new(Default::default())),
    };

    let _ = hir_instance; // This will invoke the drop method when _ goes out of scope
}

#[test]
fn test_drop_repetition_with_non_empty_sub_containing_alternation() {
    use crate::{hir::Hir, hir::HirKind, hir::Repetition};

    let nested_alternation = Hir {
        kind: HirKind::Alternation(vec![
            Hir {
                kind: HirKind::Literal(Default::default()),
                props: Properties(Box::new(Default::default())),
            },
            Hir {
                kind: HirKind::Concat(vec![
                    Hir {
                        kind: HirKind::Literal(Default::default()),
                        props: Properties(Box::new(Default::default())),
                    },
                    Hir {
                        kind: HirKind::Class(Default::default()),
                        props: Properties(Box::new(Default::default())),
                    },
                ]),
                props: Properties(Box::new(Default::default())),
            },
        ]),
        props: Properties(Box::new(Default::default())),
    };

    let repetition_expression = Repetition {
        min: 0,
        max: Some(3),
        greedy: true,
        sub: Box::new(nested_alternation),
    };

    let hir_instance = Hir {
        kind: HirKind::Repetition(repetition_expression),
        props: Properties(Box::new(Default::default())),
    };

    let _ = hir_instance; // This will invoke the drop method when _ goes out of scope
}

