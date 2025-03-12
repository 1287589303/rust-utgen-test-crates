// Answer 0

#[test]
fn test_pop_class_binary_lhs() {
    struct TestAst;

    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::default(),
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::default()))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::default()))),
    };

    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal::default()));

    let induct = ClassFrame::BinaryLHS {
        op: &op,
        lhs: &ClassSet::Item(ClassSetItem::Literal(Literal::default())),
        rhs: &rhs,
    };

    let visitor = HeapVisitor::new();

    let result = visitor.pop_class(induct);
}

#[test]
fn test_pop_class_binary_lhs_with_empty_rhs() {
    struct TestAst;

    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::default(),
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::default()))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::default()))),
    };

    let induct = ClassFrame::BinaryLHS {
        op: &op,
        lhs: &ClassSet::Item(ClassSetItem::Literal(Literal::default())),
        rhs: &ClassSet::Item(ClassSetItem::Empty(Span::default())),
    };

    let visitor = HeapVisitor::new();

    let result = visitor.pop_class(induct);
}

#[test]
fn test_pop_class_binary_lhs_with_different_op() {
    struct TestAst;

    let op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::DifferentOperation,
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::default()))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::default()))),
    };

    let rhs = ClassSet::Item(ClassSetItem::Literal(Literal::default()));

    let induct = ClassFrame::BinaryLHS {
        op: &op,
        lhs: &ClassSet::Item(ClassSetItem::Literal(Literal::default())),
        rhs: &rhs,
    };

    let visitor = HeapVisitor::new();

    let result = visitor.pop_class(induct);
}

