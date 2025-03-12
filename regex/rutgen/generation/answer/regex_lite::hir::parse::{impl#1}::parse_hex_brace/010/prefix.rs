// Answer 0

#[test]
fn test_parse_hex_brace_invalid_hex_digit() {
    let parser = {
        let capture_names = RefCell::new(vec![String::from("group1")]);
        let config = Config {
            nest_limit: 100,
            flags: Flags::default(),
        };
        let pattern = "{GHIJK"; // Invalid hex, but setup for test
        Parser {
            config,
            pattern,
            depth: Cell::new(0),
            pos: Cell::new(0),
            char: Cell::new(Some('{')),
            capture_index: Cell::new(0),
            flags: RefCell::new(Flags::default()),
            capture_names,
        }
    };

    parser.parse_hex_brace().unwrap_err();
}

#[test]
fn test_parse_hex_brace_empty_scratch() {
    let parser = {
        let capture_names = RefCell::new(vec![String::from("group1")]);
        let config = Config {
            nest_limit: 100,
            flags: Flags::default(),
        };
        let pattern = "{"; // Just opening brace
        Parser {
            config,
            pattern,
            depth: Cell::new(0),
            pos: Cell::new(0),
            char: Cell::new(Some('{')),
            capture_index: Cell::new(0),
            flags: RefCell::new(Flags::default()),
            capture_names,
        }
    };

    parser.parse_hex_brace().unwrap_err();
}

#[test]
fn test_parse_hex_brace_invalid_u32_conversion() {
    let parser = {
        let capture_names = RefCell::new(vec![String::from("group1")]);
        let config = Config {
            nest_limit: 100,
            flags: Flags::default(),
        };
        let pattern = "{GHIJKL}"; // Invalid hex representation, shouldn't parse to u32
        Parser {
            config,
            pattern,
            depth: Cell::new(0),
            pos: Cell::new(0),
            char: Cell::new(Some('}')),
            capture_index: Cell::new(0),
            flags: RefCell::new(Flags::default()),
            capture_names,
        }
    };

    parser.parse_hex_brace().unwrap_err();
}

