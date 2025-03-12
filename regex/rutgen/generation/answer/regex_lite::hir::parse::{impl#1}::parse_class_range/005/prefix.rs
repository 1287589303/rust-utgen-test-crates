// Answer 0

#[test]
fn test_parse_class_range_single_char_with_closing_bracket() {
    let flags = Flags::default();
    let config = Config { nest_limit: 10, flags };
    let pattern = "a]"; // single character followed by a closing bracket
    let mut union = Vec::new();
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_class_range(&mut union);
}

#[test]
fn test_parse_class_range_single_char_with_space() {
    let flags = Flags::default();
    let config = Config { nest_limit: 10, flags };
    let pattern = "a "; // single character followed by a space
    let mut union = Vec::new();
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_class_range(&mut union);
}

#[test]
fn test_parse_class_range_invalid_range_min_greater_than_max() {
    let flags = Flags::default();
    let config = Config { nest_limit: 10, flags };
    let pattern = "b-c"; // invalid range where start is greater than end
    let mut union = Vec::new();
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('b')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_class_range(&mut union);
}

#[test]
fn test_parse_class_range_empty_class() {
    let flags = Flags::default();
    let config = Config { nest_limit: 10, flags };
    let pattern = "[]"; // empty character class
    let mut union = Vec::new();
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some(']')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_class_range(&mut union);
}

#[test]
fn test_parse_class_range_invalid_closing_bracket() {
    let flags = Flags::default();
    let config = Config { nest_limit: 10, flags };
    let pattern = "a-"; // single character followed by '-'
    let mut union = Vec::new();
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(flags),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_class_range(&mut union);
}

