// Answer 0

#[test]
fn test_lift_common_prefix_empty_prefix() {
    let first = Hir {
        kind: HirKind::Concat(vec![]), // Empty prefix
        props: Properties {},
    };
    let second = Hir {
        kind: HirKind::Concat(vec![Hir {
            kind: HirKind::Literal(Literal::from("test")),
            props: Properties {},
        }]),
        props: Properties {},
    };
    
    let hirs = vec![first, second];
    let _ = lift_common_prefix(hirs);
}

#[test]
fn test_lift_common_prefix_multiple_empty_prefix() {
    let first = Hir {
        kind: HirKind::Concat(vec![]), // Empty prefix
        props: Properties {},
    };
    let second = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::from("example")),
                props: Properties {},
            },
        ]),
        props: Properties {},
    };
    let third = Hir {
        kind: HirKind::Concat(vec![
            Hir {
                kind: HirKind::Literal(Literal::from("sample")),
                props: Properties {},
            },
        ]),
        props: Properties {},
    };
    
    let hirs = vec![first, second, third];
    let _ = lift_common_prefix(hirs);
}

