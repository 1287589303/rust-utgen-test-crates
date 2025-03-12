// Answer 0

#[test]
fn test_span_class_bracketed() {
    let span = Span { start: Position(0), end: Position(5) };
    let class_bracketed = ClassBracketed {
        span: span.clone(),
        negated: false,
        kind: ClassSet::Normal(vec!['a', 'b', 'c']),
    };
    let ast = Ast::ClassBracketed(Box::new(class_bracketed));
    let _result = ast.span();
}

#[test]
fn test_span_class_bracketed_negated() {
    let span = Span { start: Position(10), end: Position(15) };
    let class_bracketed = ClassBracketed {
        span: span.clone(),
        negated: true,
        kind: ClassSet::Normal(vec!['x', 'y', 'z']),
    };
    let ast = Ast::ClassBracketed(Box::new(class_bracketed));
    let _result = ast.span();
}

#[test]
fn test_span_class_bracketed_edge_case() {
    let span = Span { start: Position(0), end: Position(0) };  // Edge case: start equals end
    let class_bracketed = ClassBracketed {
        span: span.clone(),
        negated: false,
        kind: ClassSet::Normal(vec![]),  // Empty character set
    };
    let ast = Ast::ClassBracketed(Box::new(class_bracketed));
    let _result = ast.span();
}

