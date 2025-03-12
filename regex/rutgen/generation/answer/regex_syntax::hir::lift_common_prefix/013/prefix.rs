// Answer 0

#[test]
fn test_lift_common_prefix_success() {
    let hirs = vec![
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal("foo".into()),
                    props: Properties::default(),
                },
                Hir {
                    kind: HirKind::Literal("bar".into()),
                    props: Properties::default(),
                },
            ]),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal("foo".into()),
                    props: Properties::default(),
                },
                Hir {
                    kind: HirKind::Literal("baz".into()),
                    props: Properties::default(),
                },
            ]),
            props: Properties::default(),
        },
    ];

    let _result = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_multiple_levels() {
    let hirs = vec![
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal("abc".into()),
                    props: Properties::default(),
                },
                Hir {
                    kind: HirKind::Literal("xyz".into()),
                    props: Properties::default(),
                },
            ]),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal("abc".into()),
                    props: Properties::default(),
                },
                Hir {
                    kind: HirKind::Literal("123".into()),
                    props: Properties::default(),
                },
            ]),
            props: Properties::default(),
        },
    ];

    let _result = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_edge_case() {
    let hirs = vec![
        Hir {
            kind: HirKind::Concat(vec![Hir {
                kind: HirKind::Literal("123".into()),
                props: Properties::default(),
            }]),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Concat(vec![Hir {
                kind: HirKind::Literal("456".into()),
                props: Properties::default(),
            }]),
            props: Properties::default(),
        },
    ];

    let _result = lift_common_prefix(hirs);
}

