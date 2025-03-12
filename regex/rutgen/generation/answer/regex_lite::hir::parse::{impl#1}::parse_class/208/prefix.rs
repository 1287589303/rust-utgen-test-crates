// Answer 0

#[test]
fn test_parse_class_with_symmetric_difference() {
    let pattern = "[~-]";
    let config = Config {
        size_limit: None,
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

    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let result = parser.parse_class();
}

