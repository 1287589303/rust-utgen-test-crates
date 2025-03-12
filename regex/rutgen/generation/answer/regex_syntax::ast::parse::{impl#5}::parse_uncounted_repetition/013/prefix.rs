// Answer 0

#[test]
fn test_parse_uncounted_repetition_with_star() {
    let position_start = Position { offset: 0, line: 1, column: 1 };
    let position_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(position_start, position_end);
    
    let ast = Ast::literal(Box::new(ast::Literal { span: span.clone() }));
    
    let concat = Concat {
        span: span.clone(),
        asts: vec![ast],
    };

    let parser = ParserI {
        parser: Box::new(Parser {
            pos: Cell::new(position_start),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            empty_min_range: true,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        }),
        pattern: "*",
    };

    parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrMore).unwrap();
}

#[test]
fn test_parse_uncounted_repetition_with_question() {
    let position_start = Position { offset: 0, line: 1, column: 1 };
    let position_end = Position { offset: 1, line: 1, column: 2 };
    let span = Span::new(position_start, position_end);
    
    let ast = Ast::literal(Box::new(ast::Literal { span: span.clone() }));
    
    let concat = Concat {
        span: span.clone(),
        asts: vec![ast],
    };

    let parser = ParserI {
        parser: Box::new(Parser {
            pos: Cell::new(position_start),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: true,
            initial_ignore_whitespace: false,
            empty_min_range: true,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        }),
        pattern: "?",
    };

    parser.parse_uncounted_repetition(concat, ast::RepetitionKind::ZeroOrOne).unwrap();
}

