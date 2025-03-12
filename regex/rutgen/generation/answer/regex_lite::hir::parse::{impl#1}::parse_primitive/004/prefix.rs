// Answer 0

#[test]
fn test_parse_primitive_dot() {
    let config = Config {
        nest_limit: 10,
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
    
    let _result = parser.parse_primitive();
}

#[test]
fn test_parse_primitive_dot_with_dot_matches_new_line() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: true,
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
    
    let _result = parser.parse_primitive();
}

