// Answer 0

#[test]
fn test_parse_primitive_with_dollar_anchor() {
    let parser = {
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
        let pattern = "$";
        Parser {
            config,
            pattern,
            depth: Cell::new(0),
            pos: Cell::new(0),
            char: Cell::new(Some('$')),
            capture_index: Cell::new(0),
            flags: RefCell::new(Flags::default()),
            capture_names: RefCell::new(vec![]),
        }
    };

    parser.parse_primitive().unwrap();
}

#[test]
fn test_parse_primitive_with_dollar_anchor_multiline() {
    let parser = {
        let config = Config {
            nest_limit: 10,
            flags: Flags {
                case_insensitive: false,
                multi_line: true,
                dot_matches_new_line: false,
                swap_greed: false,
                crlf: false,
                ignore_whitespace: false,
            },
        };
        let pattern = "$";
        Parser {
            config,
            pattern,
            depth: Cell::new(0),
            pos: Cell::new(0),
            char: Cell::new(Some('$')),
            capture_index: Cell::new(0),
            flags: RefCell::new(Flags::default()),
            capture_names: RefCell::new(vec![]),
        }
    };

    parser.parse_primitive().unwrap();
}

