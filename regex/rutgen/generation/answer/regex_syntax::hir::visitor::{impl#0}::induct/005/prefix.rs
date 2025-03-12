// Answer 0

#[test]
fn test_induct_concat_non_empty() {
    let sub_hir1 = Hir {
        kind: HirKind::Literal(Literal::from("a")),
        props: Properties::default(),
    };
    let sub_hir2 = Hir {
        kind: HirKind::Literal(Literal::from("b")),
        props: Properties::default(),
    };
    let hir = Hir {
        kind: HirKind::Concat(vec![sub_hir1.clone(), sub_hir2.clone()]),
        props: Properties::default(),
    };
    
    let mut visitor = HeapVisitor::new();
    visitor.induct(&hir);
}

#[test]
fn test_induct_concat_multiple_elements() {
    let sub_hir1 = Hir {
        kind: HirKind::Literal(Literal::from("x")),
        props: Properties::default(),
    };
    let sub_hir2 = Hir {
        kind: HirKind::Literal(Literal::from("y")),
        props: Properties::default(),
    };
    let sub_hir3 = Hir {
        kind: HirKind::Literal(Literal::from("z")),
        props: Properties::default(),
    };
    let hir = Hir {
        kind: HirKind::Concat(vec![sub_hir1.clone(), sub_hir2.clone(), sub_hir3.clone()]),
        props: Properties::default(),
    };
    
    let mut visitor = HeapVisitor::new();
    visitor.induct(&hir);
}

#[test]
fn test_induct_concat_three_elements() {
    let sub_hir1 = Hir {
        kind: HirKind::Literal(Literal::from("1")),
        props: Properties::default(),
    };
    let sub_hir2 = Hir {
        kind: HirKind::Literal(Literal::from("2")),
        props: Properties::default(),
    };
    let sub_hir3 = Hir {
        kind: HirKind::Literal(Literal::from("3")),
        props: Properties::default(),
    };
    let hir = Hir {
        kind: HirKind::Concat(vec![sub_hir1.clone(), sub_hir2.clone(), sub_hir3.clone()]),
        props: Properties::default(),
    };
    
    let mut visitor = HeapVisitor::new();
    visitor.induct(&hir);
}

