// Answer 0

#[test]
fn test_span_dot_empty_range() {
    let span = Span { start: Position(0), end: Position(0) };
    let primitive = Primitive::Dot(span);
    let result = primitive.span();
}

#[test]
fn test_span_dot_non_empty_range() {
    let span = Span { start: Position(0), end: Position(10) };
    let primitive = Primitive::Dot(span);
    let result = primitive.span();
}

#[test]
fn test_span_dot_boundary_case() {
    let span = Span { start: Position(5), end: Position(5) };
    let primitive = Primitive::Dot(span);
    let result = primitive.span();
}

#[test]
fn test_span_dot_large_range() {
    let span = Span { start: Position(100), end: Position(200) };
    let primitive = Primitive::Dot(span);
    let result = primitive.span();
}

