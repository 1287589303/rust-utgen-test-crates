// Answer 0

#[test]
fn test_visit_with_concat() {
    struct TestVisitor {
        output: Vec<Ast>,
    }

    impl Visitor for TestVisitor {
        type Output = Vec<Ast>;
        type Err = ();
        
        fn start(&mut self) {}
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), Self::Err> {
            self.output.push(ast.clone());
            Ok(())
        }
        fn visit_post(&mut self, ast: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Concat(Box::new(Concat {
        expressions: vec![
            Ast::Literal(Box::new(Literal("a".to_string()))),
            Ast::Repetition(Box::new(Repetition {
                expr: Box::new(Ast::Literal(Box::new(Literal("b".to_string())))),
            })),
        ],
    }));

    let mut visitor = TestVisitor { output: vec![] };
    let mut heap_visitor = HeapVisitor::new();
    
    heap_visitor.visit(&ast, visitor).unwrap();
}

#[test]
fn test_visit_with_group() {
    struct TestVisitor {
        output: Vec<Ast>,
    }

    impl Visitor for TestVisitor {
        type Output = Vec<Ast>;
        type Err = ();
        
        fn start(&mut self) {}
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), Self::Err> {
            self.output.push(ast.clone());
            Ok(())
        }
        fn visit_post(&mut self, ast: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Group(Box::new(Group {
        inner: vec![
            Ast::Literal(Box::new(Literal("a".to_string()))),
            Ast::Alternation(Box::new(Alternation {
                branches: vec![
                    Ast::Literal(Box::new(Literal("b".to_string()))),
                    Ast::Literal(Box::new(Literal("c".to_string()))),
                ],
            })),
        ],
    }));

    let mut visitor = TestVisitor { output: vec![] };
    let mut heap_visitor = HeapVisitor::new();

    heap_visitor.visit(&ast, visitor).unwrap();
}

#[test]
fn test_visit_with_repetition() {
    struct TestVisitor {
        output: Vec<Ast>,
    }

    impl Visitor for TestVisitor {
        type Output = Vec<Ast>;
        type Err = ();
        
        fn start(&mut self) {}
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), Self::Err> {
            self.output.push(ast.clone());
            Ok(())
        }
        fn visit_post(&mut self, ast: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Repetition(Box::new(Repetition {
        expr: Box::new(Ast::Group(Box::new(Group {
            inner: vec![
                Ast::Literal(Box::new(Literal("a".to_string()))),
            ],
        }))),
    }));

    let mut visitor = TestVisitor { output: vec![] };
    let mut heap_visitor = HeapVisitor::new();
    
    heap_visitor.visit(&ast, visitor).unwrap();
}

#[test]
fn test_visit_with_alternation() {
    struct TestVisitor {
        output: Vec<Ast>,
    }

    impl Visitor for TestVisitor {
        type Output = Vec<Ast>;
        type Err = ();
        
        fn start(&mut self) {}
        fn visit_pre(&mut self, ast: &Ast) -> Result<(), Self::Err> {
            self.output.push(ast.clone());
            Ok(())
        }
        fn visit_post(&mut self, ast: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_alternation_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
        fn visit_concat_in(&mut self) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let ast = Ast::Alternation(Box::new(Alternation {
        branches: vec![
            Ast::Literal(Box::new(Literal("a".to_string()))),
            Ast::Literal(Box::new(Literal("b".to_string()))),
        ],
    }));

    let mut visitor = TestVisitor { output: vec![] };
    let mut heap_visitor = HeapVisitor::new();
    
    heap_visitor.visit(&ast, visitor).unwrap();
}

