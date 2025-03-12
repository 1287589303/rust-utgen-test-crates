// Answer 0

#[test]
fn test_parse_set_class_with_nested_class_and_unclosed() {
    let parser = {
        let mut pos = Cell::new(Position::new());
        let comments = RefCell::new(vec![]);
        let stack_class = RefCell::new(vec![ClassState::Open { union: ast::ClassSetUnion { span: Span { start: pos.get(), end: pos.get() }, items: vec![] }, set: ast::ClassBracketed { span: Span { start: pos.get(), end: pos.get() }, negated: false, kind: ClassSet::Normal } }]);
        let parser = Parser {
            pos,
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments,
            stack_group: RefCell::new(vec![]),
            stack_class,
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        };
        parser
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "[[a-z]]",
    };

    let result = parser_i.parse_set_class();
}

#[test]
fn test_parse_set_class_with_ascii_class_and_err() {
    let parser = {
        let mut pos = Cell::new(Position::new());
        let comments = RefCell::new(vec![]);
        let stack_class = RefCell::new(vec![ClassState::Open { union: ast::ClassSetUnion { span: Span { start: pos.get(), end: pos.get() }, items: vec![] }, set: ast::ClassBracketed { span: Span { start: pos.get(), end: pos.get() }, negated: false, kind: ClassSet::Normal } }]);
        let parser = Parser {
            pos,
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments,
            stack_group: RefCell::new(vec![]),
            stack_class,
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        };
        parser
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "[[:alpha:]]&[[::digit:]]",
    };

    let result = parser_i.parse_set_class();
} 

#[test]
fn test_parse_set_class_with_invalid_range() {
    let parser = {
        let mut pos = Cell::new(Position::new());
        let comments = RefCell::new(vec![]);
        let stack_class = RefCell::new(vec![ClassState::Open { union: ast::ClassSetUnion { span: Span { start: pos.get(), end: pos.get() }, items: vec![] }, set: ast::ClassBracketed { span: Span { start: pos.get(), end: pos.get() }, negated: false, kind: ClassSet::Normal } }]);
        let parser = Parser {
            pos,
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments,
            stack_group: RefCell::new(vec![]),
            stack_class,
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        };
        parser
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "[a--b]",
    };

    let result = parser_i.parse_set_class();
} 

#[test]
fn test_parse_set_class_with_intersection_and_valid() {
    let parser = {
        let mut pos = Cell::new(Position::new());
        let comments = RefCell::new(vec![]);
        let stack_class = RefCell::new(vec![ClassState::Open { union: ast::ClassSetUnion { span: Span { start: pos.get(), end: pos.get() }, items: vec![] }, set: ast::ClassBracketed { span: Span { start: pos.get(), end: pos.get() }, negated: false, kind: ClassSet::Normal } }]);
        let parser = Parser {
            pos,
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: false,
            ignore_whitespace: Cell::new(false),
            comments,
            stack_group: RefCell::new(vec![]),
            stack_class,
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        };
        parser
    };

    let parser_i = ParserI {
        parser: &parser,
        pattern: "[a-z&&[a]]",
    };

    let result = parser_i.parse_set_class();
}

