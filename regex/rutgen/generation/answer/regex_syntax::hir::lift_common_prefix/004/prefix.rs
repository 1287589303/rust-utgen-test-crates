// Answer 0

#[test]
fn test_lift_common_prefix_with_non_concat_second_element() {
    let first_hir = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::new("foo".to_string())),
                props: Properties::default(),
            },
            Hir {
                kind: HirKind::Literal(Literal::new("bar".to_string())),
                props: Properties::default(),
            },
        ]),
        props: Properties::default(),
    };

    let second_hir = Hir {
        kind: HirKind::Look(Look { /* fields */ }),
        props: Properties::default(),
    };

    let hirs = vec![first_hir, second_hir];

    let _result = lift_common_prefix(hirs);
}

