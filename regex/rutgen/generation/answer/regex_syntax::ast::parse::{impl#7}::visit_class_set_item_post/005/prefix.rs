// Answer 0

#[test]
fn test_visit_class_set_item_post_empty() {
    let ast_item = ast::ClassSetItem::Empty(Span::default());
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "",
    };
    let mut limiter = NestLimiter::new(&parser);
    let _result = limiter.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_literal() {
    let ast_item = ast::ClassSetItem::Literal(Literal::default());
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "",
    };
    let mut limiter = NestLimiter::new(&parser);
    let _result = limiter.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_ascii() {
    let ast_item = ast::ClassSetItem::Ascii(ClassAscii::default());
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "",
    };
    let mut limiter = NestLimiter::new(&parser);
    let _result = limiter.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_unicode() {
    let ast_item = ast::ClassSetItem::Unicode(ClassUnicode::default());
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "",
    };
    let mut limiter = NestLimiter::new(&parser);
    let _result = limiter.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_range() {
    let ast_item = ast::ClassSetItem::Range(ClassSetRange::default());
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "",
    };
    let mut limiter = NestLimiter::new(&parser);
    let _result = limiter.visit_class_set_item_post(&ast_item);
}

#[test]
fn test_visit_class_set_item_post_perl() {
    let ast_item = ast::ClassSetItem::Perl(ClassPerl::default());
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::default()),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "",
    };
    let mut limiter = NestLimiter::new(&parser);
    let _result = limiter.visit_class_set_item_post(&ast_item);
}

