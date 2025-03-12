// Answer 0

#[test]
fn test_visit_post_empty() {
    let ast = Ast::Empty(Box::new(Span::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_flags() {
    let ast = Ast::Flags(Box::new(SetFlags::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_literal() {
    let ast = Ast::Literal(Box::new(Literal::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_dot() {
    let ast = Ast::Dot(Box::new(Span::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_assertion() {
    let ast = Ast::Assertion(Box::new(Assertion::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_class_unicode() {
    let ast = Ast::ClassUnicode(Box::new(ClassUnicode::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_class_perl() {
    let ast = Ast::ClassPerl(Box::new(ClassPerl::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_class_bracketed() {
    let ast = Ast::ClassBracketed(Box::new(ClassBracketed::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_repetition() {
    let ast = Ast::Repetition(Box::new(Repetition::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_group() {
    let ast = Ast::Group(Box::new(Group::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_alternation() {
    let ast = Ast::Alternation(Box::new(Alternation::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

#[test]
fn test_visit_post_concat() {
    let ast = Ast::Concat(Box::new(Concat::default()));
    struct TestVisitor;
    impl Visitor for TestVisitor {
        type Output = ();
        type Err = ();
        fn finish(self) -> Result<Self::Output, Self::Err> {
            Ok(())
        }
        fn start(&mut self) {}
    }
    let mut visitor = TestVisitor;
    visitor.visit_post(&ast).unwrap();
}

