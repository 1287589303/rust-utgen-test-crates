// Answer 0

#[test]
fn test_span_literal_valid_range() {
    let span = Span { start: Position(0), end: Position(5) };
    let literal = Literal { span, kind: LiteralKind::SomeKind, c: 'a' };
    let primitive = Primitive::Literal(literal);
    let result = primitive.span();
}

#[test]
fn test_span_literal_edge_case() {
    let span = Span { start: Position(3), end: Position(3) };
    let literal = Literal { span, kind: LiteralKind::SomeKind, c: 'b' };
    let primitive = Primitive::Literal(literal);
    let result = primitive.span();
}

