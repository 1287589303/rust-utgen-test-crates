// Answer 0

#[test]
fn test_parse_class_range_valid_character() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "a";
    let union: Vec<hir::ClassRange> = vec![];
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

    parser.parse_class_range(&mut union).unwrap();
}


#[test]
fn test_parse_class_range_valid_range() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "a-b";
    let union: Vec<hir::ClassRange> = vec![];
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

    parser.parse_class_range(&mut union).unwrap();
}


#[test]
fn test_parse_class_range_character_not_dash() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "a c";
    let union: Vec<hir::ClassRange> = vec![];
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

    parser.parse_class_range(&mut union).unwrap();
}

