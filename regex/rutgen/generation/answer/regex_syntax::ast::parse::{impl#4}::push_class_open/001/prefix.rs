// Answer 0

#[test]
fn test_push_class_open_valid_input_with_error() {
    let parent_union = ast::ClassSetUnion {
        span: Span { start: 0, end: 1 },
        items: vec![],
    };
    
    let mut parser = Parser {
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
    };

    let parser_i = ParserI::new(&parser, "[a-z"); // ensure parser is positioned at '['
    let _ = parser_i.push_class_open(parent_union);
}

#[test]
#[should_panic]
fn test_push_class_open_char_not_opening_bracket() {
    let parent_union = ast::ClassSetUnion {
        span: Span { start: 0, end: 1 },
        items: vec![],
    };

    let mut parser = Parser {
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
    };

    let parser_i = ParserI::new(&parser, "abc"); // ensure parser is positioned at a char that is not '['
    let _ = parser_i.push_class_open(parent_union);
}

