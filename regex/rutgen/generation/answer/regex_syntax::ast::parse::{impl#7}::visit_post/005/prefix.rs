// Answer 0

#[test]
fn test_visit_post_class_bracketed() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "[a-zA-Z]",
    };

    let mut nest_limiter = NestLimiter::new(&parser_i);
    nest_limiter.depth = 1;

    let class_bracketed = Ast::ClassBracketed(Box::new(ast::ClassBracketed {
        items: vec![],
    }));

    let _ = nest_limiter.visit_post(&class_bracketed);
}

#[test]
fn test_visit_post_class_bracketed_with_nested() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "[[a-zA-Z]]",
    };

    let mut nest_limiter = NestLimiter::new(&parser_i);
    nest_limiter.depth = 2;

    let class_bracketed_nested = Ast::ClassBracketed(Box::new(ast::ClassBracketed {
        items: vec![
            // Include nested character class as an example
            ast::ClassSetItem::ClassBracketed(Box::new(ast::ClassBracketed {
                items: vec![],
            })),
        ],
    }));

    let _ = nest_limiter.visit_post(&class_bracketed_nested);
}

#[test]
fn test_visit_post_class_bracketed_multiple_items() {
    let parser = Parser {
        pos: Cell::new(Position::default()),
        capture_index: Cell::new(0),
        nest_limit: 5,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(false),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "[a-zA-Z0-9]",
    };

    let mut nest_limiter = NestLimiter::new(&parser_i);
    nest_limiter.depth = 3;

    let class_bracketed_multiple = Ast::ClassBracketed(Box::new(ast::ClassBracketed {
        items: vec![
            ast::ClassSetItem::Literal(Box::new(ast::Literal {
                character: 'a',
            })),
            ast::ClassSetItem::Literal(Box::new(ast::Literal {
                character: 'b',
            })),
        ],
    }));

    let _ = nest_limiter.visit_post(&class_bracketed_multiple);
}

