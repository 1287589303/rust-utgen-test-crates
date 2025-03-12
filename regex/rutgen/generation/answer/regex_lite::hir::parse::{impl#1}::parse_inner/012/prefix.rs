// Answer 0

#[test]
fn test_parse_inner_with_uncounted_repetition_plus() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "(abc+)";
    let mut parser = Parser {
        config,
        pattern,
        depth: Cell::new(1),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.char.set(Some('+'));
    let _ = parser.parse_inner();
}

#[test]
fn test_parse_inner_with_uncounted_repetition_star() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "(abc*)";
    let mut parser = Parser {
        config,
        pattern,
        depth: Cell::new(1),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.char.set(Some('*'));
    let _ = parser.parse_inner();
}

#[test]
fn test_parse_inner_with_uncounted_repetition_question() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "(abc?)";
    let mut parser = Parser {
        config,
        pattern,
        depth: Cell::new(1),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    parser.char.set(Some('?'));
    let _ = parser.parse_inner();
}

