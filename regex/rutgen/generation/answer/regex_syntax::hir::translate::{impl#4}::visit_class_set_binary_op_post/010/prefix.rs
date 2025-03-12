// Answer 0

#[test]
fn test_visit_class_set_binary_op_post_difference_case_insensitive() {
    struct DummyVisitor {
        flags: Flags,
        stack: RefCell<Vec<HirFrame>>,
    }

    impl Visitor for DummyVisitor {
        type Output = Hir;
        type Err = Error;

        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(Hir { kind: HirKind::Literal, props: Properties::default() })
        }

        fn flags(&self) -> Flags {
            self.flags
        }

        fn pop(&self) -> Option<HirFrame> {
            self.stack.borrow_mut().pop()
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
    }

    let mut visitor = DummyVisitor {
        flags: Flags { unicode: Some(false), case_insensitive: Some(true), ..Flags::default() },
        stack: RefCell::new(vec![
            HirFrame::ClassBytes(ClassBytes::new(vec![])),
            HirFrame::ClassBytes(ClassBytes::new(vec![])),
            HirFrame::ClassBytes(ClassBytes::new(vec![])),
        ]),
    };

    let op = ast::ClassSetBinaryOp {
        kind: ast::ClassSetBinaryOpKind::Difference,
        span: Span::default(),
        lhs: Box::new(ClassSet::Item(ClassSetItem::Literal(ast::Literal::default()))),
        rhs: Box::new(ClassSet::Item(ClassSetItem::Literal(ast::Literal::default()))),
    };

    let _ = visitor.visit_class_set_binary_op_post(&op);
}

