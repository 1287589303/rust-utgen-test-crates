// Answer 0

#[test]
fn test_span_range_valid() {
    let span = Span { start: Position(0), end: Position(10) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Char, c: 'a' };
    let end_literal = Literal { span: span.clone(), kind: LiteralKind::Char, c: 'z' };
    let range = ClassSetRange { span: span.clone(), start: start_literal.clone(), end: end_literal.clone() };
    let item = ClassSetItem::Range(range);
    let result = item.span();
}

#[test]
fn test_span_range_equal_literals() {
    let span = Span { start: Position(0), end: Position(5) };
    let literal = Literal { span: span.clone(), kind: LiteralKind::Char, c: 'a' };
    let range = ClassSetRange { span: span.clone(), start: literal.clone(), end: literal.clone() };
    let item = ClassSetItem::Range(range);
    let result = item.span();
}

#[test]
fn test_span_range_invalid() {
    let span = Span { start: Position(0), end: Position(5) };
    let start_literal = Literal { span: span.clone(), kind: LiteralKind::Char, c: 'z' };
    let end_literal = Literal { span: span.clone(), kind: LiteralKind::Char, c: 'a' };
    let range = ClassSetRange { span: span.clone(), start: start_literal.clone(), end: end_literal.clone() };
    let item = ClassSetItem::Range(range);
    let result = item.span();
}

