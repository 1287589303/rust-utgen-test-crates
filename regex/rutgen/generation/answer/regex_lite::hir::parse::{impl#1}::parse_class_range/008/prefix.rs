// Answer 0

#[test]
fn test_parse_class_range_with_valid_range() {
    let config = Config {
        size_limit: None,
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "a-z";
    let mut union: Vec<hir::ClassRange> = vec![];

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
fn test_parse_class_range_with_escaped_range() {
    let config = Config {
        size_limit: None,
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "\\w-\\w";
    let mut union: Vec<hir::ClassRange> = vec![];

    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('\\')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    parser.parse_class_range(&mut union).unwrap();
}

#[test]
fn test_parse_class_range_with_single_character() {
    let config = Config {
        size_limit: None,
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "a-";
    let mut union: Vec<hir::ClassRange> = vec![];

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
fn test_parse_class_range_with_invalid_range() {
    let config = Config {
        size_limit: None,
        nest_limit: 10,
        flags: Flags::default(),
    };
    let pattern = "z-a"; // Invalid range.
    let mut union: Vec<hir::ClassRange> = vec![];

    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('z')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let result = parser.parse_class_range(&mut union);
    assert!(result.is_err());
}

