// Answer 0

#[test]
fn test_parse_counted_repetition_valid_case() {
    let config = Config { nest_limit: 100, flags: Flags::default() };
    let pattern = "{2,5}";
    let mut concat: Vec<Hir> = vec![Hir::char('a')]; // Assume Hir::char is a valid single character Hir expression.
    
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

    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_with_max() {
    let config = Config { nest_limit: 100, flags: Flags::default() };
    let pattern = "{1,3}";
    let mut concat: Vec<Hir> = vec![Hir::char('b')]; // Assume Hir::char is a valid single character Hir expression.
    
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

    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_single_value() {
    let config = Config { nest_limit: 100, flags: Flags::default() };
    let pattern = "{5}";
    let mut concat: Vec<Hir> = vec![Hir::char('c')]; // Assume Hir::char is a valid single character Hir expression.
    
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

    let _ = parser.parse_counted_repetition(concat);
}

#[test]
#[should_panic]
fn test_parse_counted_repetition_empty_concat() {
    let config = Config { nest_limit: 100, flags: Flags::default() };
    let pattern = "{1,3}";
    let concat: Vec<Hir> = vec![]; // Empty concat should cause an error.
    
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

    let _ = parser.parse_counted_repetition(concat);
}

#[test]
fn test_parse_counted_repetition_min_greater_than_max() {
    let config = Config { nest_limit: 100, flags: Flags::default() };
    let pattern = "{5,2}"; // Invalid case where min is greater than max
    let mut concat: Vec<Hir> = vec![Hir::char('d')]; // Assume Hir::char is a valid single character Hir expression.
    
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

    let _ = parser.parse_counted_repetition(concat);
}

