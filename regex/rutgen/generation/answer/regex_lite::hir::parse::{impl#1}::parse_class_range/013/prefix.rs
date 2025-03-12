// Answer 0

#[test]
fn test_parse_class_range_valid_range() {
    let config = Config { nest_limit: 5, flags: Flags::default() };
    let pattern = "a-z";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let mut union: Vec<hir::ClassRange> = vec![];
    let _result = parser.parse_class_range(&mut union);
}

#[test]
fn test_parse_class_range_equal_range() {
    let config = Config { nest_limit: 5, flags: Flags::default() };
    let pattern = "a-a"; // Both items are the same
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let mut union: Vec<hir::ClassRange> = vec![];
    let _result = parser.parse_class_range(&mut union);
}

#[test]
fn test_parse_class_range_with_different_chars() {
    let config = Config { nest_limit: 5, flags: Flags::default() };
    let pattern = "a-b";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let mut union: Vec<hir::ClassRange> = vec![];
    let _result = parser.parse_class_range(&mut union);
}

