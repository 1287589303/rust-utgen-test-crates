// Answer 0

#[test]
fn test_visit_class_set_item_post_literal_verbatim() {
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    };
    let ast = ast::ClassSetItem::Literal(literal);
    let writer = Writer { wtr: String::new() };
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal_meta() {
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Meta,
        c: 'b',
    };
    let ast = ast::ClassSetItem::Literal(literal);
    let writer = Writer { wtr: String::new() };
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal_superfluous() {
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Superfluous,
        c: 'c',
    };
    let ast = ast::ClassSetItem::Literal(literal);
    let writer = Writer { wtr: String::new() };
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal_octal() {
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Octal,
        c: '7',
    };
    let ast = ast::ClassSetItem::Literal(literal);
    let writer = Writer { wtr: String::new() };
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal_hex_fixed_unicode_short() {
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::HexFixed(ast::HexLiteralKind::UnicodeShort),
        c: 'd',
    };
    let ast = ast::ClassSetItem::Literal(literal);
    let writer = Writer { wtr: String::new() };
    writer.visit_class_set_item_post(&ast).unwrap();
}

#[test]
fn test_visit_class_set_item_post_literal_special() {
    let literal = ast::Literal {
        span: Span::default(),
        kind: ast::LiteralKind::Special(ast::SpecialLiteralKind::Tab),
        c: '\t',
    };
    let ast = ast::ClassSetItem::Literal(literal);
    let writer = Writer { wtr: String::new() };
    writer.visit_class_set_item_post(&ast).unwrap();
}

