// Answer 0

#[test]
fn test_into_ast_empty() {
    let span = Span { start: Position(0), end: Position(0) };
    let alternation = Alternation { span, asts: vec![] };
    alternation.into_ast();
}

#[test]
fn test_into_ast_single() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Ast::literal(Box::new(Literal { value: 'a' })); // Placeholder for a literal
    let alternation = Alternation { span, asts: vec![literal] };
    alternation.into_ast();
}

#[test]
fn test_into_ast_multiple() {
    let span = Span { start: Position(0), end: Position(2) };
    let literal1 = Ast::literal(Box::new(Literal { value: 'a' })); // Placeholder for a literal
    let literal2 = Ast::literal(Box::new(Literal { value: 'b' })); // Placeholder for another literal
    let alternation = Alternation { span, asts: vec![literal1, literal2] };
    alternation.into_ast();
}

