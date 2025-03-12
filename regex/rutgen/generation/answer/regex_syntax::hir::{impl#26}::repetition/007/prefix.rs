// Answer 0

#[test]
fn test_repetition_min_zero_with_max_one() {
    let sub_hir = Hir {
        kind: HirKind::Literal(Literal::from("a")),
        props: Properties::empty(),
    };
    let repetition = Repetition {
        min: 0,
        max: Some(1),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    let _properties = Properties::repetition(&repetition);
}

#[test]
fn test_repetition_min_zero_with_max_none() {
    let sub_hir = Hir {
        kind: HirKind::Literal(Literal::from("b")),
        props: Properties::empty(),
    };
    let repetition = Repetition {
        min: 0,
        max: None,
        greedy: false,
        sub: Box::new(sub_hir),
    };
    let _properties = Properties::repetition(&repetition);
}

#[test]
fn test_repetition_min_zero_static_captures_zero() {
    let sub_hir = Hir {
        kind: HirKind::Literal(Literal::from("c")),
        props: Properties::empty(),
    };
    let repetition = Repetition {
        min: 0,
        max: Some(3),
        greedy: true,
        sub: Box::new(sub_hir),
    };
    let _properties = Properties::repetition(&repetition);
}

