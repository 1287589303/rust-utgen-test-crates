// Answer 0

#[test]
fn test_push_group_with_flags() {
    let span = Span { start: Position(0), end: Position(1) };
    let set_flags = SetFlags { span, flags: Flags { span, items: vec![] } };
    let ast_flags = Ast::Flags(Box::new(set_flags));

    let concat = Concat {
        span,
        asts: vec![],
    };

    let parser = Parser {
        pos: Cell::new(Position(0)),
        capture_index: Cell::new(0),
        nest_limit: 10,
        octal: false,
        initial_ignore_whitespace: false,
        empty_min_range: false,
        ignore_whitespace: Cell::new(true),
        comments: RefCell::new(vec![]),
        stack_group: RefCell::new(vec![]),
        stack_class: RefCell::new(vec![]),
        capture_names: RefCell::new(vec![]),
        scratch: RefCell::new(String::new()),
    };

    let parser_i = ParserI::new(&parser, "(?i)");

    parser_i.push_group(concat).unwrap();
}

#[test]
fn test_push_group_with_negated_flags() {
    let span = Span { start: Position(0), end: Position(2) };
    let set_flags = SetFlags { span, flags: Flags { span, items: vec![FlagsItem::negation(ast::Flag::IgnoreWhitespace)] } };
    let ast_flags = Ast::Flags(Box::new(set_flags));

    let concat = Concat {
        span,
        asts: vec![],
    };

    let parser = Parser {
        pos: Cell::new(Position(0)),
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
    };

    let parser_i = ParserI::new(&parser, "(?x)");

    parser_i.push_group(concat).unwrap();
}

