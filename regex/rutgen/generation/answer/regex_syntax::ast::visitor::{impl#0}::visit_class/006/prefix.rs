// Answer 0

#[test]
fn test_visit_class_with_inductive_item_and_binary_op() {
    struct TestVisitor {
        binary_op_called: bool,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            self.binary_op_called = true;
            Err(())
        }
    }

    let mut visitor = TestVisitor { binary_op_called: false };
    
    let ast = ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: ClassSet::Item(Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: Span::default(),
            negated: false,
            kind: ClassSet::BinaryOp(Box::new(ClassSetBinaryOp {
                span: Span::default(),
                kind: ClassSetBinaryOpKind::Union,
                lhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Single('a')))),
                rhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Single('b')))),
            })),
        }))),
    };

    let mut visitor_impl = HeapVisitor::new();
    visitor_impl.visit_class(&ast, &mut visitor).ok();
}

#[test]
fn test_visit_class_with_empty_tail() {
    struct TestVisitor {
        binary_op_called: bool,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<(), Self::Err> {
            self.binary_op_called = true;
            Err(())
        }
    }

    let mut visitor = TestVisitor { binary_op_called: false };

    let ast = ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: ClassSet::BinaryOp(Box::new(ClassSetBinaryOp {
            span: Span::default(),
            kind: ClassSetBinaryOpKind::Intersection,
            lhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Single('c')))),
            rhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Single('d')))),
        })),
    };

    let mut visitor_impl = HeapVisitor::new();
    visitor_impl.visit_class(&ast, &mut visitor).ok();
}

