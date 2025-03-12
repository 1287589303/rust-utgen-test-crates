// Answer 0

#[test]
fn test_induct_with_empty_hir() {
    let hir = Hir {
        kind: HirKind::Empty,
        props: Properties::default(),
    };
    let mut visitor = HeapVisitor::new();
    let result = visitor.induct(&hir);
}

#[test]
fn test_induct_with_literal_hir() {
    let hir = Hir {
        kind: HirKind::Literal(Literal::from("test")),
        props: Properties::default(),
    };
    let mut visitor = HeapVisitor::new();
    let result = visitor.induct(&hir);
}

