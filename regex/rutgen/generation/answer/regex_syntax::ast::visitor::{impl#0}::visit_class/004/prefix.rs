// Answer 0

#[test]
fn test_visit_class_with_union() {
    struct TestVisitor {
        result: Result<(), ()>,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&self, _: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let lhs = ClassSet::BinaryOp(Box::new(ClassSetBinaryOp {
        span: Span::new(0, 5),
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ClassSet::Item(Box::new(ClassSetItem::Single('a')))),
        rhs: Box::new(ClassSet::Item(Box::new(ClassSetItem::Single('b')))),
    }));

    let rhs = ClassSet::BinaryOp(Box::new(ClassSetBinaryOp {
        span: Span::new(6, 10),
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ClassSet::Item(Box::new(ClassSetItem::Single('c')))),
        rhs: Box::new(ClassSet::Item(Box::new(ClassSetItem::Single('d')))),
    }));

    let ast = ClassBracketed {
        span: Span::new(0, 10),
        negated: false,
        kind: ClassSet::BinaryOp(Box::new(ClassSetBinaryOp {
            span: Span::new(0, 10),
            kind: ClassSetBinaryOpKind::Union,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })),
    };

    let mut visitor = TestVisitor { result: Ok(()) };
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit_class(&ast, &mut visitor).unwrap();
}

#[test]
fn test_visit_class_with_binary_operation() {
    struct TestVisitor {
        result: Result<(), ()>,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&self, _: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = ClassBracketed {
        span: Span::new(0, 10),
        negated: false,
        kind: ClassSet::BinaryOp(Box::new(ClassSetBinaryOp {
            span: Span::new(0, 10),
            kind: ClassSetBinaryOpKind::Intersect,
            lhs: Box::new(ClassSet::Item(Box::new(ClassSetItem::Single('a')))),
            rhs: Box::new(ClassSet::Item(Box::new(ClassSetItem::Single('b')))),
        })),
    };

    let mut visitor = TestVisitor { result: Ok(()) };
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit_class(&ast, &mut visitor).unwrap();
}

#[test]
fn test_visit_class_with_empty_stack_class() {
    struct TestVisitor {
        result: Result<(), ()>,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&self, _: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = ClassBracketed {
        span: Span::new(0, 10),
        negated: false,
        kind: ClassSet::Item(Box::new(ClassSetItem::Single('a'))),
    };

    let mut visitor = TestVisitor { result: Ok(()) };
    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.stack_class.pop(); // ensure the stack is empty

    heap_visitor.visit_class(&ast, &mut visitor).unwrap();
}

