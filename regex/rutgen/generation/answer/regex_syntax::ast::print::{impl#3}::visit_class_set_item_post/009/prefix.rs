// Answer 0

#[test]
fn test_visit_class_set_item_post_range_valid_case() {
    let start_char = 'a';
    let end_char = 'z';
    let valid_span = Span::new(0, 5);
    let start_span = Span::new(0, 1);
    let end_span = Span::new(4, 5);

    let literal_start = Literal {
        span: start_span,
        kind: LiteralKind::Verbatim,
        c: start_char,
    };

    let literal_end = Literal {
        span: end_span,
        kind: LiteralKind::Verbatim,
        c: end_char,
    };

    let range = ClassSetRange {
        span: valid_span,
        start: literal_start,
        end: literal_end,
    };

    let class_set_item = ast::ClassSetItem::Range(range);

    let writer = Writer {
        wtr: String::new(),
    };

    writer.visit_class_set_item_post(&class_set_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_range_valid_case2() {
    let start_char = '1';
    let end_char = '9';
    let valid_span = Span::new(0, 5);
    let start_span = Span::new(0, 1);
    let end_span = Span::new(4, 5);

    let literal_start = Literal {
        span: start_span,
        kind: LiteralKind::Verbatim,
        c: start_char,
    };

    let literal_end = Literal {
        span: end_span,
        kind: LiteralKind::Verbatim,
        c: end_char,
    };

    let range = ClassSetRange {
        span: valid_span,
        start: literal_start,
        end: literal_end,
    };

    let class_set_item = ast::ClassSetItem::Range(range);

    let writer = Writer {
        wtr: String::new(),
    };

    writer.visit_class_set_item_post(&class_set_item).unwrap();
}

#[test]
fn test_visit_class_set_item_post_range_valid_case3() {
    let start_char = '!';
    let end_char = '#';
    let valid_span = Span::new(0, 5);
    let start_span = Span::new(0, 1);
    let end_span = Span::new(4, 5);

    let literal_start = Literal {
        span: start_span,
        kind: LiteralKind::Verbatim,
        c: start_char,
    };

    let literal_end = Literal {
        span: end_span,
        kind: LiteralKind::Verbatim,
        c: end_char,
    };

    let range = ClassSetRange {
        span: valid_span,
        start: literal_start,
        end: literal_end,
    };

    let class_set_item = ast::ClassSetItem::Range(range);

    let writer = Writer {
        wtr: String::new(),
    };

    writer.visit_class_set_item_post(&class_set_item).unwrap();
}

