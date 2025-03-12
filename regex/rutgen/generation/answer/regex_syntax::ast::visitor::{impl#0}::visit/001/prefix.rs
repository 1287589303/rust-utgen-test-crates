// Answer 0

#[test]
fn test_visit_empty_ast() {
    let ast = Ast::Empty(Box::new(Span::new(0, 0)));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_flags_ast() {
    let ast = Ast::Flags(Box::new(SetFlags::new()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_literal_ast() {
    let ast = Ast::Literal(Box::new(Literal::new('a')));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_dot_ast() {
    let ast = Ast::Dot(Box::new(Span::new(0, 1)));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_assertion_ast() {
    let ast = Ast::Assertion(Box::new(Assertion::new()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_class_unicode_ast() {
    let ast = Ast::ClassUnicode(Box::new(ClassUnicode::new()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_class_perl_ast() {
    let ast = Ast::ClassPerl(Box::new(ClassPerl::new()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_class_bracketed_ast() {
    let ast = Ast::ClassBracketed(Box::new(ClassBracketed::new(vec![])));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_repetition_ast() {
    let ast = Ast::Repetition(Box::new(Repetition::new()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_group_ast() {
    let ast = Ast::Group(Box::new(Group::new()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_alternation_ast() {
    let ast = Ast::Alternation(Box::new(Alternation::new(vec![])));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

#[test]
fn test_visit_concat_ast() {
    let ast = Ast::Concat(Box::new(Concat::new(vec![])));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn start(&mut self) {}
        fn visit_pre(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Err(())
        }
        fn visit_post(&mut self, _: &Ast) -> Result<(), Self::Err> {
            Ok(())
        }
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
    }
    let mut visitor = TestVisitor;
    let mut heap_visitor = HeapVisitor::new();
    let _ = heap_visitor.visit(&ast, visitor);
}

