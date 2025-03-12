// Answer 0

#[test]
fn test_has_subexprs_alternation_with_single_expression() {
    let span = Span { start: Position::start(), end: Position::end() };
    let group = Group { span: span.clone(), kind: GroupKind::Default, ast: Box::new(Ast::literal(Literal::new(span.clone(), 'a'))) };
    let alternation = Alternation { span, asts: vec![Box::new(Ast::group(group))] };
    let ast = Ast::alternation(alternation);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_alternation_with_multiple_expressions() {
    let span = Span { start: Position::start(), end: Position::end() };
    let concat = Concat { span: span.clone(), asts: vec![Box::new(Ast::literal(Literal::new(span.clone(), 'a'))), Box::new(Ast::literal(Literal::new(span.clone(), 'b')))] };
    let alternation = Alternation { span, asts: vec![Box::new(Ast::concat(concat)), Box::new(Ast::dot(span.clone()))] };
    let ast = Ast::alternation(alternation);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_alternation_with_mixed_expressions() {
    let span = Span { start: Position::start(), end: Position::end() };
    let repetition = Repetition { min: 1, max: Some(3), greedy: true, sub: Box::new(Ast::literal(Literal::new(span.clone(), 'a'))) };
    let group = Group { span: span.clone(), kind: GroupKind::Default, ast: Box::new(Ast::assertion(Assertion { span: span.clone(), kind: AssertionKind::WordBoundary })) };
    let alternation = Alternation { span, asts: vec![Box::new(Ast::repetition(repetition)), Box::new(Ast::group(group))] };
    let ast = Ast::alternation(alternation);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_alternation_with_empty_expressions() {
    let span = Span { start: Position::start(), end: Position::end() };
    let alternation = Alternation { span, asts: vec![Box::new(Ast::empty(span.clone()))] };
    let ast = Ast::alternation(alternation);
    ast.has_subexprs();
}

#[test]
fn test_has_subexprs_alternation_with_duplicates() {
    let span = Span { start: Position::start(), end: Position::end() };
    let group = Group { span: span.clone(), kind: GroupKind::Default, ast: Box::new(Ast::literal(Literal::new(span.clone(), 'x'))) };
    let alternation = Alternation { span, asts: vec![Box::new(Ast::group(group.clone())), Box::new(Ast::group(group))] };
    let ast = Ast::alternation(alternation);
    ast.has_subexprs();
}

