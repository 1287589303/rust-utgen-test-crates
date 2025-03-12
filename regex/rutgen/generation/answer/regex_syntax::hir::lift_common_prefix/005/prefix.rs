// Answer 0

#[test]
fn test_lift_common_prefix_case1() {
    let hirs = vec![
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("abc")),
                    props: Properties::default(),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("def")),
                    props: Properties::default(),
                },
            ]),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("abc")),
                    props: Properties::default(),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("xyz")),
                    props: Properties::default(),
                },
            ]),
            props: Properties::default(),
        },
    ];
    let _ = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_case2() {
    let hirs = vec![
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("hello")),
                    props: Properties::default(),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("world")),
                    props: Properties::default(),
                },
            ]),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("hello")),
                    props: Properties::default(),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("rust")),
                    props: Properties::default(),
                },
            ]),
            props: Properties::default(),
        },
    ];
    let _ = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_case3() {
    let hirs = vec![
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("123")),
                    props: Properties::default(),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("456")),
                    props: Properties::default(),
                },
            ]),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::from("123")),
                    props: Properties::default(),
                },
                Hir {
                    kind: HirKind::Literal(Literal::from("789")),
                    props: Properties::default(),
                },
            ]),
            props: Properties::default(),
        },
    ];
    let _ = lift_common_prefix(hirs);
}

