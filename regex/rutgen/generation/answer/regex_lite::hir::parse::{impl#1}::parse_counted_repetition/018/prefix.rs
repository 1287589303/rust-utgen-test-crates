// Answer 0

#[test]
fn test_parse_counted_repetition_valid_case() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "{2}";
    let mut parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let concat = vec![Hir::char('a')];
    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_with_greedy() {
    let config = Config { nest_limit: 10, flags: Flags { swap_greed: false, ..Flags::default() }};
    let pattern = "{2}";
    let mut parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let concat = vec![Hir::char('b')];
    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_empty_after_reading() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "{2}";
    let mut parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let concat = vec![Hir::char('c')];
    let result = parser.parse_counted_repetition(concat);
}

#[test]
#[should_panic(expected = "expected closing brace")]
fn test_parse_counted_repetition_unclosed() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "{2";
    let mut parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let concat = vec![Hir::char('d')];
    let result = parser.parse_counted_repetition(concat);
}

