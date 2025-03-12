// Answer 0

#[test]
fn test_parse_counted_repetition_empty_concat() {
    let config = Config { size_limit: None };
    let pattern = "{2,5}"; // Example pattern to test
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    
    let concat: Vec<Hir> = vec![];
    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_sub_missing() {
    let config = Config { size_limit: None };
    let pattern = "{2,5}"; // Example pattern to test
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let concat: Vec<Hir> = vec![];
    let result = parser.parse_counted_repetition(concat);
}

