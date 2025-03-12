// Answer 0

#[test]
fn test_parse_primitive_anchor_start() {
    let config = Config {
        size_limit: None,
        nest_limit: 100,
        flags: Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        },
    };
    let pattern = "^";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('^')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_dot() {
    let config = Config {
        size_limit: None,
        nest_limit: 100,
        flags: Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        },
    };
    let pattern = ".";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('.')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_anchor_end() {
    let config = Config {
        size_limit: None,
        nest_limit: 100,
        flags: Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        },
    };
    let pattern = "$";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('$')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_escape() {
    let config = Config {
        size_limit: None,
        nest_limit: 100,
        flags: Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        },
    };
    let pattern = "\\";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('\\')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_character() {
    let config = Config {
        size_limit: None,
        nest_limit: 100,
        flags: Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            swap_greed: false,
            crlf: false,
            ignore_whitespace: false,
        },
    };
    let pattern = "a"; // any valid character
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_primitive();
}

