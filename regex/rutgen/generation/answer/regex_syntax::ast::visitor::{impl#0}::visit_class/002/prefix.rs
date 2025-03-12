// Answer 0

#[test]
fn test_visit_class_binary_op() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let binary_op = ClassSetBinaryOp {
        span: Span::default(),
        kind: ClassSetBinaryOpKind::Union,
        lhs: Box::new(ClassSet::BinaryOp(Box::new(ClassSetBinaryOp {
            span: Span::default(),
            kind: ClassSetBinaryOpKind::Union,
            lhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
                span: Span::default(),
                negated: false,
                kind: ClassSet::Empty,
            })))),
            rhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
                span: Span::default(),
                negated: false,
                kind: ClassSet::Empty,
            }))),
}))),
        rhs: Box::new(ClassSet::Item(Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
            span: Span::default(),
            negated: false,
            kind: ClassSet::Empty,
        }))),
    };

    let class_bracketed = ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: ClassSet::BinaryOp(Box::new(binary_op)),
    };

    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();
    
    heap_visitor.visit_class(&class_bracketed, &mut visitor).unwrap();
}

#[test]
fn test_visit_class_union_with_multiple_items() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class_set_binary_op_in(&mut self, _op: &ClassSetBinaryOp) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }

    let union = ast::ClassSet::Union(ast::Union {
        items: vec![
            Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
                span: Span::default(),
                negated: false,
                kind: ClassSet::Empty,
            })),
            Box::new(ast::ClassSetItem::Union(ast::Union {
                items: vec![
                    Box::new(ast::ClassSetItem::Bracketed(ClassBracketed {
                        span: Span::default(),
                        negated: false,
                        kind: ClassSet::Empty,
                    }))
                ],
            })),
        ],
    });

    let class_bracketed = ClassBracketed {
        span: Span::default(),
        negated: false,
        kind: union,
    };

    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();
    
    heap_visitor.visit_class(&class_bracketed, &mut visitor).unwrap();
}

