// Answer 0

#[test]
fn test_span_dot() {
    let span = Span { start: Position(0), end: Position(1) };
    let ast = Ast::dot(span.clone());
    let returned_span = ast.span();
}

#[test]
fn test_span_dot_minimal() {
    let span = Span { start: Position(0), end: Position(0) };
    let ast = Ast::dot(span.clone());
    let returned_span = ast.span();
}

#[test]
fn test_span_dot_large_range() {
    let span = Span { start: Position(0), end: Position(1000) };
    let ast = Ast::dot(span.clone());
    let returned_span = ast.span();
}

#[test]
fn test_span_dot_with_start_equals_end() {
    let span = Span { start: Position(5), end: Position(5) };
    let ast = Ast::dot(span.clone());
    let returned_span = ast.span();
}

