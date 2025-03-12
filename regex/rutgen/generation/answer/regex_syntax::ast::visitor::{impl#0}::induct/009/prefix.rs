// Answer 0

#[test]
fn test_induct_class_bracketed() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class(&mut self, _class: &ClassBracketed) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let span = Span::new(1, 5);
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Normal(vec![]),
    };
    let ast = Ast::ClassBracketed(Box::new(class_bracketed));
    
    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_class_bracketed_negated() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class(&mut self, _class: &ClassBracketed) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let span = Span::new(1, 5);
    let class_bracketed = ClassBracketed {
        span,
        negated: true,
        kind: ClassSet::Normal(vec![]),
    };
    let ast = Ast::ClassBracketed(Box::new(class_bracketed));

    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_class_bracketed_with_items() {
    struct MockVisitor;

    impl Visitor for MockVisitor {
        type Output = ();
        type Err = ();

        fn visit_class(&mut self, _class: &ClassBracketed) -> Result<(), Self::Err> {
            Ok(())
        }
    }

    let span = Span::new(1, 5);
    let class_bracketed = ClassBracketed {
        span,
        negated: false,
        kind: ClassSet::Union(vec![]),
    };
    let ast = Ast::ClassBracketed(Box::new(class_bracketed));

    let mut visitor = MockVisitor;
    let mut heap_visitor = HeapVisitor::new();

    let result = heap_visitor.induct(&ast, &mut visitor);
}

