// Answer 0

#[test]
fn test_visit_class_set_item_pre_empty() {
    struct TestVisitor {
        flags: Flags,
        stack: RefCell<Vec<HirFrame>>,
    }

    impl Visitor for TestVisitor {
        type Output = Hir;
        type Err = Error;

        fn finish(self) -> Result<Hir> {
            Ok(Hir { kind: HirKind::Empty, props: Properties::default() })
        }

        fn flags(&self) -> Flags {
            self.flags
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
    }

    let test_ast = ast::ClassSetItem::Empty(Span::default());
    let visitor = TestVisitor {
        flags: Flags::default(),
        stack: RefCell::new(vec![]),
    };

    let _ = visitor.visit_class_set_item_pre(&test_ast);
}

#[test]
fn test_visit_class_set_item_pre_literal() {
    struct TestVisitor {
        flags: Flags,
        stack: RefCell<Vec<HirFrame>>,
    }

    impl Visitor for TestVisitor {
        type Output = Hir;
        type Err = Error;

        fn finish(self) -> Result<Hir> {
            Ok(Hir { kind: HirKind::Empty, props: Properties::default() })
        }

        fn flags(&self) -> Flags {
            self.flags
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
    }

    let test_ast = ast::ClassSetItem::Literal(Literal::default());
    let visitor = TestVisitor {
        flags: Flags::default(),
        stack: RefCell::new(vec![]),
    };

    let _ = visitor.visit_class_set_item_pre(&test_ast);
}

