// Answer 0

#[test]
fn test_drop_concat_with_look() {
    let _ = {
        let inner_hir = Hir {
            kind: HirKind::Look,
            props: Properties(Box::new(PropertiesI {})),
        };
        let concat_hir = Hir {
            kind: HirKind::Concat(vec![inner_hir]),
            props: Properties(Box::new(PropertiesI {})),
        };
        concat_hir
    };
}

#[test]
fn test_drop_concat_with_literal() {
    let _ = {
        let inner_hir = Hir {
            kind: HirKind::Literal(Literal::from("test")),
            props: Properties(Box::new(PropertiesI {})),
        };
        let concat_hir = Hir {
            kind: HirKind::Concat(vec![inner_hir]),
            props: Properties(Box::new(PropertiesI {})),
        };
        concat_hir
    };
}

#[test]
fn test_drop_concat_with_class() {
    let _ = {
        let inner_hir = Hir {
            kind: HirKind::Class(Class::from('a')),
            props: Properties(Box::new(PropertiesI {})),
        };
        let concat_hir = Hir {
            kind: HirKind::Concat(vec![inner_hir]),
            props: Properties(Box::new(PropertiesI {})),
        };
        concat_hir
    };
}

#[test]
fn test_drop_concat_with_multiple_expressions() {
    let _ = {
        let inner_hir1 = Hir {
            kind: HirKind::Look,
            props: Properties(Box::new(PropertiesI {})),
        };
        let inner_hir2 = Hir {
            kind: HirKind::Literal(Literal::from("test")),
            props: Properties(Box::new(PropertiesI {})),
        };
        let concat_hir = Hir {
            kind: HirKind::Concat(vec![inner_hir1, inner_hir2]),
            props: Properties(Box::new(PropertiesI {})),
        };
        concat_hir
    };
}

