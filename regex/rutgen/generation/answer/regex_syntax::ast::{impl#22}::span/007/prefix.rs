// Answer 0

#[test]
fn test_span_literal_with_empty_span() {
    let position_start = Position { offset: 0 };
    let position_end = Position { offset: 0 };
    let span = Span { start: position_start, end: position_end };
    let literal = Literal { span, kind: LiteralKind::SomeKind, c: 'a' };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

#[test]
fn test_span_literal_with_same_start_and_end() {
    let position_start = Position { offset: 5 };
    let position_end = Position { offset: 5 };
    let span = Span { start: position_start, end: position_end };
    let literal = Literal { span, kind: LiteralKind::SomeKind, c: 'b' };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

#[test]
fn test_span_literal_with_valid_span() {
    let position_start = Position { offset: 10 };
    let position_end = Position { offset: 20 };
    let span = Span { start: position_start, end: position_end };
    let literal = Literal { span, kind: LiteralKind::SomeKind, c: 'c' };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

#[test]
fn test_span_literal_with_max_offset() {
    let position_start = Position { offset: usize::max_value() as u64 };
    let position_end = Position { offset: usize::max_value() as u64 };
    let span = Span { start: position_start, end: position_end };
    let literal = Literal { span, kind: LiteralKind::SomeKind, c: 'd' };
    let item = ClassSetItem::Literal(literal);
    let _ = item.span();
}

