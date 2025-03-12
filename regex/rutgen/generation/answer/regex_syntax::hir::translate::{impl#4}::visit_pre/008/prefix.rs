// Answer 0

#[test]
fn test_visit_pre_class_bracketed_non_unicode() {
    struct TestVisitor {
        flags: Flags,
        stack: RefCell<Vec<HirFrame>>,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self {
                flags: Flags {
                    unicode: Some(false),
                    ..Flags::default()
                },
                stack: RefCell::new(vec![]),
            }
        }

        fn flags(&self) -> &Flags {
            &self.flags
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
    }

    let mut visitor = TestVisitor::new();
    let ast = Ast::ClassBracketed(ast::ClassBracketed::new(vec![]));
    let _ = visitor.visit_pre(&ast);
}

#[test]
fn test_visit_pre_class_bracketed_unicode() {
    struct TestVisitor {
        flags: Flags,
        stack: RefCell<Vec<HirFrame>>,
    }

    impl TestVisitor {
        fn new() -> Self {
            Self {
                flags: Flags {
                    unicode: Some(true),
                    ..Flags::default()
                },
                stack: RefCell::new(vec![]),
            }
        }

        fn flags(&self) -> &Flags {
            &self.flags
        }

        fn push(&self, frame: HirFrame) {
            self.stack.borrow_mut().push(frame);
        }
    }

    let mut visitor = TestVisitor::new();
    let ast = Ast::ClassBracketed(ast::ClassBracketed::new(vec![]));
    let _ = visitor.visit_pre(&ast);
}

