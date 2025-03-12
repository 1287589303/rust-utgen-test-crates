// Answer 0

#[test]
fn test_visit_with_concat_frame() {
    struct DummyVisitor {
        pre_visit_count: usize,
        post_visit_count: usize,
    }

    impl DummyVisitor {
        fn new() -> Self {
            Self { pre_visit_count: 0, post_visit_count: 0 }
        }
    }

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.pre_visit_count += 1;
            Ok(())
        }
        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.post_visit_count += 1;
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let mut visitor = DummyVisitor::new();

    let child1 = Ast::Literal(Box::new(Literal { /* initialization */ }));
    let child2 = Ast::Literal(Box::new(Literal { /* initialization */ }));
    let concat_node = Ast::Concat(Box::new(Concat {
        head: Box::new(child1),
        tail: vec![child2],
    }));

    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.stack.push((&concat_node, Frame::Concat {
        head: &concat_node,
        tail: &vec![child2],
    }));

    let _ = heap_visitor.visit(&concat_node, visitor);
}

#[test]
fn test_visit_with_concatenation_path() {
    struct DummyVisitor {
        pre_visit_count: usize,
        post_visit_count: usize,
    }

    impl DummyVisitor {
        fn new() -> Self {
            Self { pre_visit_count: 0, post_visit_count: 0 }
        }
    }

    impl Visitor for DummyVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let mut visitor = DummyVisitor::new();

    let child1 = Ast::Literal(Box::new(Literal { /* initialization */ }));
    let child2 = Ast::Literal(Box::new(Literal { /* initialization */ }));
    let concat_node = Ast::Concat(Box::new(Concat {
        head: Box::new(child1),
        tail: vec![child2],
    }));

    let mut heap_visitor = HeapVisitor::new();
    heap_visitor.stack.push((&concat_node, Frame::Concat {
        head: &concat_node,
        tail: &vec![child2],
    }));

    let _ = heap_visitor.visit(&concat_node, visitor);
}

