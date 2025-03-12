// Answer 0

#[test]
fn test_parse_set_class_item_verbatim_literal_small() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::from(0)),
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
            scratch: RefCell::new(String::from("a")),
        },
        pattern: "a",
    };
    parser.parse_set_class_item().unwrap();
}

#[test]
fn test_parse_set_class_item_verbatim_literal_large() {
    let input = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::from(0)),
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
            scratch: RefCell::new(String::from(input)),
        },
        pattern: input,
    };
    parser.parse_set_class_item().unwrap();
}

#[test]
fn test_parse_set_class_item_verbatim_literal_unicode() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::from(0)),
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
            scratch: RefCell::new(String::from("☃")),
        },
        pattern: "☃",
    };
    parser.parse_set_class_item().unwrap();
}

#[test]
fn test_parse_set_class_item_multiple_characters() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::from(0)),
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
            scratch: RefCell::new(String::from("hello")),
        },
        pattern: "hello",
    };
    parser.parse_set_class_item().unwrap();
}

#[test]
fn test_parse_set_class_item_non_special_character() {
    let parser = ParserI {
        parser: Parser {
            pos: Cell::new(Position::from(0)),
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
            scratch: RefCell::new(String::from(".*?")),
        },
        pattern: ".*?",
    };
    parser.parse_set_class_item().unwrap();
}

