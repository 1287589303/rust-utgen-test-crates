// Answer 0

#[test]
fn test_parse_set_class_range_valid() {
    let pos1 = Position { offset: 0, line: 1, column: 1 };
    let pos2 = Position { offset: 3, line: 1, column: 4 };
    let span1 = Span::new(pos1, pos1);
    let span2 = Span::new(pos2, pos2);

    let prim1 = Primitive::Literal(Literal {
        span: span1.clone(),
        kind: ast::LiteralKind::Verbatim,
        c: 'a',
    });

    let prim2 = Primitive::Literal(Literal {
        span: span2.clone(),
        kind: ast::LiteralKind::Verbatim,
        c: 'c',
    });

    let parser = Parser {
        pos: Cell::new(pos1),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: "a-c",
    };

    let _ = parser_i.parse_set_class_range();
} 

#[test]
fn test_parse_set_class_range_valid_another() {
    let pos1 = Position { offset: 0, line: 1, column: 1 };
    let pos2 = Position { offset: 5, line: 1, column: 6 };
    let span1 = Span::new(pos1, pos1);
    let span2 = Span::new(pos2, pos2);

    let prim1 = Primitive::Literal(Literal {
        span: span1.clone(),
        kind: ast::LiteralKind::Verbatim,
        c: 'g',
    });

    let prim2 = Primitive::Literal(Literal {
        span: span2.clone(),
        kind: ast::LiteralKind::Verbatim,
        c: 'j',
    });

    let parser = Parser {
        pos: Cell::new(pos1),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: "g-j",
    };

    let _ = parser_i.parse_set_class_range();
} 

#[test]
fn test_parse_set_class_range_valid_uppercase() {
    let pos1 = Position { offset: 0, line: 1, column: 1 };
    let pos2 = Position { offset: 3, line: 1, column: 4 };
    let span1 = Span::new(pos1, pos1);
    let span2 = Span::new(pos2, pos2);

    let prim1 = Primitive::Literal(Literal {
        span: span1.clone(),
        kind: ast::LiteralKind::Verbatim,
        c: 'A',
    });

    let prim2 = Primitive::Literal(Literal {
        span: span2.clone(),
        kind: ast::LiteralKind::Verbatim,
        c: 'C',
    });

    let parser = Parser {
        pos: Cell::new(pos1),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: "A-C",
    };

    let _ = parser_i.parse_set_class_range();
} 

#[test]
fn test_parse_set_class_range_valid_with_space() {
    let pos1 = Position { offset: 0, line: 1, column: 1 };
    let pos2 = Position { offset: 5, line: 1, column: 6 };
    let span1 = Span::new(pos1, pos1);
    let span2 = Span::new(pos2, pos2);

    let prim1 = Primitive::Literal(Literal {
        span: span1.clone(),
        kind: ast::LiteralKind::Verbatim,
        c: 'h',
    });

    let prim2 = Primitive::Literal(Literal {
        span: span2.clone(),
        kind: ast::LiteralKind::Verbatim,
        c: 'k',
    });

    let parser = Parser {
        pos: Cell::new(pos1),
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

    let parser_i = ParserI {
        parser: &parser,
        pattern: "h - k",
    };

    let _ = parser_i.parse_set_class_range();
}

