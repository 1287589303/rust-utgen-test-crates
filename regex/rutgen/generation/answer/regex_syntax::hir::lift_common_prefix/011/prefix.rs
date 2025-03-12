// Answer 0

#[test]
fn test_lift_common_prefix_multiple_concats() {
    let hirs = vec![
        Hir {
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
        },
        Hir {
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
        },
    ];
    
    let _ = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_with_empty_suffix() {
    let hirs = vec![
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::new("hello")),
                    props: Properties::default(),
                },
                Hir {
                    kind: HirKind::Literal(Literal::new("world")),
                    props: Properties::default(),
                },
            ]),
            props: Properties::default(),
        },
        Hir {
            kind: HirKind::Concat(vec![
                Hir {
                    kind: HirKind::Literal(Literal::new("hello")),
                    props: Properties::default(),
                }
            ]),
            props: Properties::default(),
        },
    ];
    
    let _ = lift_common_prefix(hirs);
}

