// Answer 0

#[test]
fn test_parse_group_invalid_empty_flags() {
    let config = Config {
        nest_limit: 10,
        flags: Flags::default(),
    };
    
    let pattern = "(?i:abc"; // Here, we're starting a group with a flag but not closing it.
    
    let mut parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('(')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    
    let result = parser.parse_group();
    // Expected result is an error due to empty flags
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.msg, ERR_EMPTY_FLAGS);
    }
}

