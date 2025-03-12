// Answer 0

#[test]
fn test_lift_common_prefix_case_1() {
    let hirs = vec![
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("foo")),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("bar")),
                },
            ]),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("foo")),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("baz")),
                },
            ]),
            props: Properties::default(),
        },
    ];
    let _result = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_case_2() {
    let hirs = vec![
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("abc")),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("def")),
                },
            ]),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("abc")),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("xyz")),
                },
            ]),
            props: Properties::default(),
        },
    ];
    let _result = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_case_3() {
    let hirs = vec![
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("test")),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("suite")),
                },
            ]),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("test")),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("cases")),
                },
            ]),
            props: Properties::default(),
        },
    ];
    let _result = lift_common_prefix(hirs);
}

