// Answer 0

#[test]
fn test_parse_counted_repetition_case_1() {
    let position_start = Position { offset: 0, line: 1, column: 1 };
    let position_end = Position { offset: 1, line: 1, column: 2 };

    let span = Span::new(position_start, position_end);
    let ast = Ast::Concat(Box::new(Concat { span, asts: vec![] }));

    let mut concat = Concat { span: span.clone(), asts: vec![ast.clone()] };

    let parser = Parser {
        pos: Cell::new(position_start),
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
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "{1,2}".as_ref(),
    };

    let _ = parser_i.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_case_2() {
    let position_start = Position { offset: 0, line: 1, column: 1 };
    let position_end = Position { offset: 1, line: 1, column: 2 };

    let span = Span::new(position_start, position_end);
    let ast = Ast::Concat(Box::new(Concat { span, asts: vec![] }));

    let mut concat = Concat { span: span.clone(), asts: vec![ast.clone()] };

    let parser = Parser {
        pos: Cell::new(position_start),
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
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "{,.}".as_ref(),
    };

    let _ = parser_i.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_case_3() {
    let position_start = Position { offset: 0, line: 1, column: 1 };
    let position_end = Position { offset: 1, line: 1, column: 2 };

    let span = Span::new(position_start, position_end);
    let ast = Ast::Concat(Box::new(Concat { span, asts: vec![] }));

    let mut concat = Concat { span: span.clone(), asts: vec![ast.clone()] };

    let parser = Parser {
        pos: Cell::new(position_start),
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
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "{}".as_ref(),
    };

    let _ = parser_i.parse_counted_repetition(concat);
}

