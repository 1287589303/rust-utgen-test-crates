// Answer 0

#[test]
fn test_visit_class_set_item_post_literal() {
    let item = ast::ClassSetItem::Literal(ast::Literal::new("a"));
    let parser = ParserI { parser: &Parser { pos: Cell::new(Position::default()), capture_index: Cell::new(0), nest_limit: 5, octal: true, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "a" };
    let mut limiter = NestLimiter::new(&parser);
    let _ = limiter.visit_class_set_item_post(&item);
}

#[test]
fn test_visit_class_set_item_post_ascii() {
    let item = ast::ClassSetItem::Ascii(ast::ClassAscii::new("a"));
    let parser = ParserI { parser: &Parser { pos: Cell::new(Position::default()), capture_index: Cell::new(0), nest_limit: 5, octal: true, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "a" };
    let mut limiter = NestLimiter::new(&parser);
    let _ = limiter.visit_class_set_item_post(&item);
}

#[test]
fn test_visit_class_set_item_post_unicode() {
    let item = ast::ClassSetItem::Unicode(ast::ClassUnicode::new("α"));
    let parser = ParserI { parser: &Parser { pos: Cell::new(Position::default()), capture_index: Cell::new(0), nest_limit: 5, octal: true, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "α" };
    let mut limiter = NestLimiter::new(&parser);
    let _ = limiter.visit_class_set_item_post(&item);
}

#[test]
fn test_visit_class_set_item_post_empty() {
    let item = ast::ClassSetItem::Empty(ast::Span::new(0, 1));
    let parser = ParserI { parser: &Parser { pos: Cell::new(Position::default()), capture_index: Cell::new(0), nest_limit: 5, octal: true, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut limiter = NestLimiter::new(&parser);
    let _ = limiter.visit_class_set_item_post(&item);
}

#[test]
fn test_visit_class_set_item_post_range() {
    let item = ast::ClassSetItem::Range(ast::ClassSetRange::new('a', 'z'));
    let parser = ParserI { parser: &Parser { pos: Cell::new(Position::default()), capture_index: Cell::new(0), nest_limit: 5, octal: true, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "a-z" };
    let mut limiter = NestLimiter::new(&parser);
    let _ = limiter.visit_class_set_item_post(&item);
}

#[test]
fn test_visit_class_set_item_post_perl() {
    let item = ast::ClassSetItem::Perl(ast::ClassPerl::new('d'));
    let parser = ParserI { parser: &Parser { pos: Cell::new(Position::default()), capture_index: Cell::new(0), nest_limit: 5, octal: true, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "d" };
    let mut limiter = NestLimiter::new(&parser);
    let _ = limiter.visit_class_set_item_post(&item);
}

#[test]
fn test_visit_class_set_item_post_bracketed() {
    let item = ast::ClassSetItem::Bracketed(Box::new(ast::ClassBracketed::new(vec![])));
    let parser = ParserI { parser: &Parser { pos: Cell::new(Position::default()), capture_index: Cell::new(0), nest_limit: 5, octal: true, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut limiter = NestLimiter::new(&parser);
    let _ = limiter.visit_class_set_item_post(&item);
}

#[test]
fn test_visit_class_set_item_post_union() {
    let item = ast::ClassSetItem::Union(ast::ClassSetUnion::new(vec![]));
    let parser = ParserI { parser: &Parser { pos: Cell::new(Position::default()), capture_index: Cell::new(0), nest_limit: 5, octal: true, initial_ignore_whitespace: false, empty_min_range: false, ignore_whitespace: Cell::new(false), comments: RefCell::new(vec![]), stack_group: RefCell::new(vec![]), stack_class: RefCell::new(vec![]), capture_names: RefCell::new(vec![]), scratch: RefCell::new(String::new()) }, pattern: "" };
    let mut limiter = NestLimiter::new(&parser);
    let _ = limiter.visit_class_set_item_post(&item);
}

