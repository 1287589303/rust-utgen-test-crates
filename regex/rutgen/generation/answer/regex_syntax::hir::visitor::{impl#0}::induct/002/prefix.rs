// Answer 0

#[test]
fn test_induct_alternation_empty() {
    struct TestVisitor;

    let hir = Hir {
        kind: HirKind::Alternation(vec![]),
        props: Properties::default(),
    };

    let mut visitor = HeapVisitor::new();
    let result = visitor.induct(&hir);
}

#[test]
fn test_induct_concat_empty() {
    struct TestVisitor;

    let hir = Hir {
        kind: HirKind::Concat(vec![]),
        props: Properties::default(),
    };

    let mut visitor = HeapVisitor::new();
    let result = visitor.induct(&hir);
}

