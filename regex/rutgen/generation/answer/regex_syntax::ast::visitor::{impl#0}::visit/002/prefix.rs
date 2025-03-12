// Answer 0

#[test]
fn test_visit_with_empty_ast() {
    struct MockVisitor;
    
    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();
        
        fn start(&mut self) {}
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }
    
    let ast = Ast::Empty(Box::new(Span::default()));
    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();
    
    heap_visitor.visit(&ast, visitor).unwrap();
}

#[test]
fn test_visit_with_literal_ast() {
    struct MockVisitor;
    
    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();
        
        fn start(&mut self) {}
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Literal(Box::new(Literal::new('a')));
    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit(&ast, visitor).unwrap();
}

#[test]
fn test_visit_with_concat_ast() {
    struct MockVisitor;
    
    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();
        
        fn start(&mut self) {}
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Concat(Box::new(Concat::new(vec![
        Ast::Literal(Box::new(Literal::new('a'))),
        Ast::Literal(Box::new(Literal::new('b')))
    ])));
    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit(&ast, visitor).unwrap();
} 

#[test]
fn test_visit_with_alternation_ast() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Alternation(Box::new(Alternation::new(vec![
        Ast::Literal(Box::new(Literal::new('a'))),
        Ast::Literal(Box::new(Literal::new('b')))
    ])));
    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit(&ast, visitor).unwrap();
} 

#[test]
fn test_visit_with_group_ast() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Group(Box::new(Group::new(Ast::Literal(Box::new(Literal::new('a'))))));
    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit(&ast, visitor).unwrap();
} 

#[test]
fn test_visit_with_repetition_ast() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn start(&mut self) {}
        fn visit_pre(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_post(&mut self, _ast: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Repetition(Box::new(Repetition::new(Ast::Literal(Box::new(Literal::new('a'))))));
    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit(&ast, visitor).unwrap();
} 

