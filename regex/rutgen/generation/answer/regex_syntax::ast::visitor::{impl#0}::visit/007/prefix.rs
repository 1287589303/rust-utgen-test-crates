// Answer 0

#[test]
fn test_visit_with_alternation() {
    struct TestVisitor {
        visited: Vec<&'static str>,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.visited.push("pre");
            Ok(())
        }

        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.visited.push("post");
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.visited.push("alternation_in");
            Err(()) // This satisfies the required err return
        }
    }

    let concat_ast = Ast::Concat(Box::new(/* Some valid Concat structure */));
    let alternation_ast = Ast::Alternation(Box::new(/* Some valid Alternation structure with concat_ast as a tail */));
    
    let mut visitor = TestVisitor { visited: vec![] };
    let mut heap_visitor = HeapVisitor::new();

    let _ = heap_visitor.visit(&alternation_ast, visitor);
}

#[test]
#[should_panic] // to indicate something unwanted occurs if you try to validate the stack state
fn test_visit_with_empty_concat() {
    struct TestVisitor {
        visited: Vec<&'static str>,
    }

    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}

        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.visited.push("pre");
            Ok(())
        }

        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            self.visited.push("post");
            Ok(())
        }

        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            self.visited.push("alternation_in");
            Ok(())
        }
    }

    let empty_concat = Ast::Concat(Box::new(/* Some valid empty Concat structure */));
    let alternation_ast = Ast::Alternation(Box::new(/* Some valid Alternation structure with empty_concat as a tail */));
    
    let mut visitor = TestVisitor { visited: vec![] };
    let mut heap_visitor = HeapVisitor::new();

    let _ = heap_visitor.visit(&alternation_ast, visitor);
}

