// Answer 0

#[test]
fn test_ast_literal_span() {
    let span = Span { start: Position(0), end: Position(10) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Char, c: 'a' };
    let ast = Ast::Literal(Box::new(literal));
    let result = ast.span();
}

#[test]
fn test_ast_literal_span_boundary_case_start_zero() {
    let span = Span { start: Position(0), end: Position(1) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Char, c: 'b' };
    let ast = Ast::Literal(Box::new(literal));
    let result = ast.span();
}

#[test]
fn test_ast_literal_span_boundary_case_large_end() {
    let span = Span { start: Position(0), end: Position(4294967295) }; // max u32
    let literal = Literal { span: span.clone(), kind: LiteralKind::Char, c: 'c' };
    let ast = Ast::Literal(Box::new(literal));
    let result = ast.span();
}

#[test]
fn test_ast_literal_span_two_bytes() {
    let span = Span { start: Position(1), end: Position(3) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Char, c: 'd' };
    let ast = Ast::Literal(Box::new(literal));
    let result = ast.span();
}

#[test]
fn test_ast_literal_span_reversed_case() {
    // This case might not be logical but it's to ensure that bounds are handled properly,
    // Creating a span that would normally be invalid, testing robustness.
    let span = Span { start: Position(5), end: Position(3) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Char, c: 'e' };
    let ast = Ast::Literal(Box::new(literal));
    let result = ast.span(); // We expect no panic here, just to ensure proper handling.
}

