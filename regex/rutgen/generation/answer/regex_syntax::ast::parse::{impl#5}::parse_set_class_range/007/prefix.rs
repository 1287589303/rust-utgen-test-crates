// Answer 0

#[test]
fn test_parse_set_class_range_valid() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 2, line: 1, column: 3 };
    
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
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
        pattern: "a-b",
    };
    
    let valid_literal1 = Primitive::Literal(Literal {
        span: Span::new(start_pos, start_pos),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    });

    let valid_literal2 = Primitive::Literal(Literal {
        span: Span::new(end_pos, end_pos),
        kind: ast::LiteralKind::Verbatim,
        c: 'b',
    });

    // Mimic the behavior of self's methods for the test context
    parser.parse_set_class_item = || Ok(valid_literal1.clone());
    parser.bump_space = || {};
    parser.is_eof = || false;
    parser.char = || '-';
    parser.peek_space = || None;
    parser.bump_and_bump_space = || true;
    parser.parse_set_class_item = || Ok(valid_literal2.clone());
    parser.into_class_literal = |_| Err(());

    let _result = parser.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_invalid() {
    let start_pos = Position { offset: 0, line: 1, column: 1 };
    let end_pos = Position { offset: 3, line: 1, column: 4 };
    
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(start_pos),
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
        pattern: "c-d",
    };
    
    let valid_literal1 = Primitive::Literal(Literal {
        span: Span::new(start_pos, start_pos),
        kind: ast::LiteralKind::Verbatim,
        c: 'c',
    });

    let valid_literal2 = Primitive::Literal(Literal {
        span: Span::new(end_pos, end_pos),
        kind: ast::LiteralKind::Verbatim,
        c: 'd',
    });

    // Mimic the behavior of self's methods for the test context
    parser.parse_set_class_item = || Ok(valid_literal1.clone());
    parser.bump_space = || {};
    parser.is_eof = || false;
    parser.char = || '-';
    parser.peek_space = || None;
    parser.bump_and_bump_space = || true;
    parser.parse_set_class_item = || Ok(valid_literal2.clone());
    parser.into_class_literal = |_| Ok(Literal {
        span: Span::new(end_pos, end_pos),
        kind: ast::LiteralKind::Verbatim,
        c: 'c', // Invalid as 'c' < 'd' should return an error
    });

    let _result = parser.parse_set_class_range();
}

