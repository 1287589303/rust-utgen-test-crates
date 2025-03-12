// Answer 0

#[test]
fn test_parse_inner_with_single_group() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "(abc)";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(1),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_inner();
}

#[test]
fn test_parse_inner_with_alternation() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "(abc)|(def)";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(1),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_inner();
}

#[test]
fn test_parse_inner_with_counted_repetition() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "(a{2,3})";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(1),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_inner();
}

#[test]
fn test_parse_inner_with_uncounted_repetition() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "(a+)";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(1),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_inner();
}

#[test]
fn test_parse_inner_with_empty_group() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "()";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(1),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let _ = parser.parse_inner();
}

