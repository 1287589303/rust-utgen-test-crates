// Answer 0

#[test]
fn test_parse_decimal_invalid_digits() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let pattern = "0abc";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('0')),
        capture_index: Cell::new(0),
        flags: RefCell::new(Flags::default()),
        capture_names: RefCell::new(vec![]),
    };
    let result = parser.parse_decimal();
    assert!(result.is_err());
    if let Err(error) = result {
        assert_eq!(error.msg, ERR_DECIMAL_INVALID);
    }
}

