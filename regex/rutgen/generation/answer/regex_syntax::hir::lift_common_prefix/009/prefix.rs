// Answer 0

#[test]
fn test_lift_common_prefix_case_1() {
    let hir_1 = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::new("foo")),
                props: Properties::default(),
            },
            Hir {
                kind: HirKind::Literal(Literal::new("bar")),
                props: Properties::default(),
            },
        ]),
        props: Properties::default(),
    };

    let hir_2 = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::new("foo")),
                props: Properties::default(),
            },
            Hir {
                kind: HirKind::Literal(Literal::new("baz")),
                props: Properties::default(),
            },
        ]),
        props: Properties::default(),
    };

    let hirs = vec![hir_1, hir_2];
    let _ = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_case_2() {
    let hir_1 = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::new("abc")),
                props: Properties::default(),
            },
            Hir {
                kind: HirKind::Literal(Literal::new("def")),
                props: Properties::default(),
            },
        ]),
        props: Properties::default(),
    };

    let hir_2 = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::new("abc")),
                props: Properties::default(),
            },
            Hir {
                kind: HirKind::Literal(Literal::new("xyz")),
                props: Properties::default(),
            },
        ]),
        props: Properties::default(),
    };

    let hir_3 = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::new("abc")),
                props: Properties::default(),
            },
            Hir {
                kind: HirKind::Literal(Literal::new("ghijkl")),
                props: Properties::default(),
            },
        ]),
        props: Properties::default(),
    };

    let hirs = vec![hir_1, hir_2, hir_3];
    let _ = lift_common_prefix(hirs);
}

