// Answer 0

#[test]
fn test_pos_with_valid_position() {
    struct TestParser {
        pos: Cell<Position>,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position { offset: 5, line: 1, column: 6 }),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                empty_min_range: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(Vec::new()),
                stack_group: RefCell::new(Vec::new()),
                stack_class: RefCell::new(Vec::new()),
                capture_names: RefCell::new(Vec::new()),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let parser_i = ParserI::new(TestParser {}, "a(bc)");
    let position = parser_i.pos();
}

#[test]
fn test_pos_with_edge_case_position_start() {
    struct TestParser {
        pos: Cell<Position>,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position { offset: 0, line: 1, column: 1 }),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                empty_min_range: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(Vec::new()),
                stack_group: RefCell::new(Vec::new()),
                stack_class: RefCell::new(Vec::new()),
                capture_names: RefCell::new(Vec::new()),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let parser_i = ParserI::new(TestParser {}, "abc");
    let position = parser_i.pos();
}

#[test]
fn test_pos_with_edge_case_position_end() {
    struct TestParser {
        pos: Cell<Position>,
    }

    impl Borrow<Parser> for TestParser {
        fn borrow(&self) -> &Parser {
            &Parser {
                pos: Cell::new(Position { offset: 2, line: 1, column: 3 }),
                capture_index: Cell::new(0),
                nest_limit: 10,
                octal: false,
                initial_ignore_whitespace: false,
                empty_min_range: false,
                ignore_whitespace: Cell::new(false),
                comments: RefCell::new(Vec::new()),
                stack_group: RefCell::new(Vec::new()),
                stack_class: RefCell::new(Vec::new()),
                capture_names: RefCell::new(Vec::new()),
                scratch: RefCell::new(String::new()),
            }
        }
    }

    let parser_i = ParserI::new(TestParser {}, "ab");
    let position = parser_i.pos();
}

