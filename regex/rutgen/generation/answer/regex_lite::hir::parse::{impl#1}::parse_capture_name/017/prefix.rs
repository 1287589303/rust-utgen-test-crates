// Answer 0

#[test]
fn test_parse_capture_name_invalid_group_name() {
    let flags = Flags::default();
    let config = Config { nest_limit: 10, flags };
    let pattern = "<>";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1), // Position right after '<'
        char: Cell::new(Some('>')), // Current character is '>'
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_capture_name();
}

#[test]
fn test_parse_capture_name_empty_group_name() {
    let flags = Flags::default();
    let config = Config { nest_limit: 10, flags };
    let pattern = "<>";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(1), // Position right after '<'
        char: Cell::new(Some('>')), // Current character is '>'
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_capture_name();
}

