// Answer 0

#[test]
fn test_visit_class_set_item_pre_literal() {
    let span = Span { start: Position::new(0), end: Position::new(1) };
    let ast_item = ast::ClassSetItem::Literal(Literal { value: 'a' });
    let parser_i = ParserI { parser: Parser { nest_limit: 10, ..Default::default() }, pattern: "test" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_range() {
    let span = Span { start: Position::new(0), end: Position::new(2) };
    let ast_item = ast::ClassSetItem::Range(ClassSetRange { start: 'a', end: 'z' });
    let parser_i = ParserI { parser: Parser { nest_limit: 10, ..Default::default() }, pattern: "test" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_unicode() {
    let span = Span { start: Position::new(0), end: Position::new(3) };
    let ast_item = ast::ClassSetItem::Unicode(ClassUnicode { name: String::from("Greek") });
    let parser_i = ParserI { parser: Parser { nest_limit: 10, ..Default::default() }, pattern: "test" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_empty() {
    let span = Span { start: Position::new(0), end: Position::new(0) };
    let ast_item = ast::ClassSetItem::Empty(span.clone());
    let parser_i = ParserI { parser: Parser { nest_limit: 10, ..Default::default() }, pattern: "test" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_ascii() {
    let span = Span { start: Position::new(0), end: Position::new(1) };
    let ast_item = ast::ClassSetItem::Ascii(ClassAscii { name: String::from("alnum") });
    let parser_i = ParserI { parser: Parser { nest_limit: 10, ..Default::default() }, pattern: "test" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

#[test]
fn test_visit_class_set_item_pre_perl() {
    let span = Span { start: Position::new(0), end: Position::new(1) };
    let ast_item = ast::ClassSetItem::Perl(ClassPerl { name: String::from("digit") });
    let parser_i = ParserI { parser: Parser { nest_limit: 10, ..Default::default() }, pattern: "test" };
    let mut limiter = NestLimiter::new(&parser_i);
    limiter.visit_class_set_item_pre(&ast_item).unwrap();
}

