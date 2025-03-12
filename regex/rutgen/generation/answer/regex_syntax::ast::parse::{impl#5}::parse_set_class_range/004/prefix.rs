// Answer 0

#[test]
fn test_parse_set_class_range_with_literal_before_dash_and_closing_bracket() {
    let prim1 = Primitive::Literal(Literal {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    });
    
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 0,
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
        pattern: "-]",
    };
    
    let _ = parser.parse_set_class_range();
}

#[test]
fn test_parse_set_class_range_with_literal_before_dash_and_another_dash() {
    let prim1 = Primitive::Literal(Literal {
        span: Span::new(Position { offset: 0, line: 1, column: 1 }, Position { offset: 1, line: 1, column: 2 }),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    });

    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
            capture_index: Cell::new(0),
            nest_limit: 0,
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
        pattern: "--",
    };

    let _ = parser.parse_set_class_range();
}

