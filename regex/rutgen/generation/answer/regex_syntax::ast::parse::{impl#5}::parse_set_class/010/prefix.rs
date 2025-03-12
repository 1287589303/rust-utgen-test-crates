// Answer 0

#[test]
fn test_parse_set_class_with_empty_class() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: true,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "[--]",
    };
    let _result = parser.parse_set_class();
}

#[test]
fn test_parse_set_class_with_nested_classes() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: true,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![
                ClassState::Open {
                    union: ast::ClassSetUnion {
                        span: Span { start: 0, end: 0 },
                        items: vec![],
                    },
                    set: ast::ClassBracketed {
                        span: Span { start: 0, end: 0 },
                        negated: false,
                        kind: ast::ClassSet::Normal,
                    },
                }
            ]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "[[--]]",
    };
    let _result = parser.parse_set_class();
}

#[test]
fn test_parse_set_class_with_invalid_range() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(0),
            capture_index: Cell::new(0),
            nest_limit: 10,
            octal: false,
            initial_ignore_whitespace: false,
            empty_min_range: true,
            ignore_whitespace: Cell::new(false),
            comments: RefCell::new(vec![]),
            stack_group: RefCell::new(vec![]),
            stack_class: RefCell::new(vec![]),
            capture_names: RefCell::new(vec![]),
            scratch: RefCell::new(String::new()),
        },
        pattern: "[a--]",
    };
    let _result = parser.parse_set_class();
}

