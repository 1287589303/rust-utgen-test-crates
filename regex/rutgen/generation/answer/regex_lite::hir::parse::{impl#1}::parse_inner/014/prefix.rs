// Answer 0

#[test]
fn test_parse_inner_valid_pattern_with_class() {
    let pattern = "[a-zA-Z0-9]+";
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('[')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let _result = parser.parse_inner();
}

#[test]
fn test_parse_inner_valid_pattern_with_nested_group_and_class() {
    let pattern = "(abc|[def])";
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };

    let _result = parser.parse_inner();
}

