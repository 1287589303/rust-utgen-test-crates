// Answer 0

#[test]
fn test_parse_counted_repetition_valid() {
    let config = Config { size_limit: None };
    let pattern = "{2,5}";
    
    let mut concat = vec![Hir::char('a')];
    
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

    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_min_max() {
    let config = Config { size_limit: None };
    let pattern = "{3,}";
    
    let mut concat = vec![Hir::char('b')];
    
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

    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_greedy() {
    let config = Config { size_limit: None };
    let pattern = "{0,3}?";
    
    let mut concat = vec![Hir::char('c')];
    
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('{')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags { swap_greed: false, ..Flags::default() }),
        capture_names: RefCell::new(vec![]),
    };

    let result = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_invalid() {
    let config = Config { size_limit: None };
    let pattern = "{5,2}";
    
    let mut concat = vec![Hir::char('d')];
    
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

    let result = parser.parse_counted_repetition(concat);
}

