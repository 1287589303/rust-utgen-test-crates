// Answer 0

#[test]
fn test_induct_concat_empty() {
    let ast = Ast::Concat(Box::new(Concat {
        span: Span::new(0, 0),
        asts: Vec::new(),
    }));
    let mut visitor = MyVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_concat_non_empty() {
    let ast_item = Ast::Literal(Box::new(Literal::new('a')));
    let ast = Ast::Concat(Box::new(Concat {
        span: Span::new(0, 1),
        asts: vec![ast_item],
    }));
    let mut visitor = MyVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor);
}

#[test]
fn test_induct_concat_multiple_items() {
    let ast_item1 = Ast::Literal(Box::new(Literal::new('a')));
    let ast_item2 = Ast::Literal(Box::new(Literal::new('b')));
    let ast = Ast::Concat(Box::new(Concat {
        span: Span::new(0, 2),
        asts: vec![ast_item1, ast_item2],
    }));
    let mut visitor = MyVisitor::new();
    let mut heap_visitor = HeapVisitor::new();
    let result = heap_visitor.induct(&ast, &mut visitor);
}

