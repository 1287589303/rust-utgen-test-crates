// Answer 0

#[test]
fn test_visit_with_concat_induction() {
    struct MockVisitor {
        pre_visit_ok: bool,
        post_visit_ok: bool,
        concat_in_called: bool,
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            if self.pre_visit_ok {
                Ok(())
            } else {
                Err(())
            }
        }

        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            if self.post_visit_ok {
                Ok(())
            } else {
                Err(())
            }
        }

        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            self.concat_in_called = true;
            Err(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast_concat = Ast::Concat(Box::new(Concat { /* initialize with test data */ }));
    let ast_repetition = Ast::Repetition(Box::new(Repetition { /* initialize with test data */ }));
    
    let mut visitor = MockVisitor {
        pre_visit_ok: true,
        post_visit_ok: true,
        concat_in_called: false,
    };

    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.stack.push((&ast_repetition, Frame::Concat {
        head: &ast_concat,
        tail: &[],
    }));

    let _ = heap_visitor.visit(&ast_concat, visitor);
}

#[test]
fn test_visit_with_concat_no_induction() {
    struct MockVisitor {
        pre_visit_ok: bool,
        post_visit_ok: bool,
        concat_in_called: bool,
    }

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            if self.pre_visit_ok {
                Ok(())
            } else {
                Err(())
            }
        }

        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            if self.post_visit_ok {
                Ok(())
            } else {
                Err(())
            }
        }

        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            self.concat_in_called = true;
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast_concat = Ast::Concat(Box::new(Concat { /* initialize with test data */ }));
    let ast_repetition = Ast::Repetition(Box::new(Repetition { /* initialize with test data */ }));

    let mut visitor = MockVisitor {
        pre_visit_ok: true,
        post_visit_ok: true,
        concat_in_called: false,
    };

    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.stack.push((&ast_repetition, Frame::Concat {
        head: &ast_concat,
        tail: &[],
    }));

    let _ = heap_visitor.visit(&ast_concat, visitor);
}

