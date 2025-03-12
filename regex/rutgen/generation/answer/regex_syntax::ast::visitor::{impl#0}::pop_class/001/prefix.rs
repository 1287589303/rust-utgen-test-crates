// Answer 0

#[test]
fn test_pop_class_binary_rhs() {
    let frame = ClassFrame::BinaryRHS {
        op: &ClassSetBinaryOp {
            span: Span::new(0, 1),
            kind: ClassSetBinaryOpKind::Union,
            lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('a')))),
            rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(Literal::new('b')))),
        },
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(frame);
}

#[test]
fn test_pop_class_binary_rhs_other_values() {
    let frame = ClassFrame::BinaryRHS {
        op: &ClassSetBinaryOp {
            span: Span::new(1, 2),
            kind: ClassSetBinaryOpKind::Intersection,
            lhs: Box::new(ClassSet::Item(ClassSetItem::Range(ClassSetRange::new('a', 'z')))),
            rhs: Box::new(ClassSet::Item(ClassSetItem::Bracketed(Box::new(ClassBracketed::new())))),
        },
    };

    let visitor = HeapVisitor::new();
    let result = visitor.pop_class(frame);
}

