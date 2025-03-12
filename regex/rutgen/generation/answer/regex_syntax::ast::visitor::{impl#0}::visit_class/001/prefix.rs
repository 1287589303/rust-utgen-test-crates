// Answer 0

#[test]
fn test_visit_class_empty_class() {
    struct MockVisitor;
    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    }
    let visitor = &mut MockVisitor;
    let ast = ClassBracketed {
        span: Span::new(0, 0),
        negated: false,
        kind: ClassSet::Item(Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: Span::new(0, 0),
            negated: false,
            kind: ClassSet::Empty,
        }))),
    };

    let mut visitor_instance = HeapVisitor::new();
    visitor_instance.visit_class(&ast, visitor).unwrap();
}

#[test]
fn test_visit_class_non_empty_class() {
    struct MockVisitor;
    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    }
    let visitor = &mut MockVisitor;
    let ast = ClassBracketed {
        span: Span::new(0, 5),
        negated: false,
        kind: ClassSet::Item(Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: Span::new(0, 5),
            negated: false,
            kind: ClassSet::Item(Box::new(ast::ClassSetItem::Literal("a".into()))),
        }))),
    };

    let mut visitor_instance = HeapVisitor::new();
    visitor_instance.visit_class(&ast, visitor).unwrap();
}

#[test]
fn test_visit_class_negated_class() {
    struct MockVisitor;
    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Ok(())
        }
    }
    let visitor = &mut MockVisitor;
    let ast = ClassBracketed {
        span: Span::new(1, 6),
        negated: true,
        kind: ClassSet::Item(Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: Span::new(1, 6),
            negated: true,
            kind: ClassSet::Item(Box::new(ast::ClassSetItem::Literal("b".into()))),
        }))),
    };

    let mut visitor_instance = HeapVisitor::new();
    visitor_instance.visit_class(&ast, visitor).unwrap();
}

#[test]
#[should_panic]
fn test_visit_class_visit_class_pre_error() {
    struct MockVisitor;
    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            Err(())
        }
    }
    let visitor = &mut MockVisitor;
    let ast = ClassBracketed {
        span: Span::new(2, 7),
        negated: false,
        kind: ClassSet::Item(Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: Span::new(2, 7),
            negated: false,
            kind: ClassSet::BinaryOp(Box::new(ClassSetBinaryOp {
                span: Span::new(2, 7),
                kind: ClassSetBinaryOpKind::Union,
                lhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Literal("c".into())))),
                rhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Literal("d".into())))),
            }))),
        }))),
    };

    let mut visitor_instance = HeapVisitor::new();
    visitor_instance.visit_class(&ast, visitor).unwrap();
}

