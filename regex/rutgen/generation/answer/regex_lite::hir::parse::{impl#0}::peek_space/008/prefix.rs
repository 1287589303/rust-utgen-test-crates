// Answer 0

#[test]
fn test_peek_space_with_ignore_whitespace() {
    let config = Config {
        nest_limit: 5,
        flags: Flags {
            ignore_whitespace: true,
            ..Default::default()
        },
    };
    let pattern = "abc# comment\nxyz";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(0),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(config.flags),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.peek_space();
}

#[test]
fn test_peek_space_with_ignore_whitespace_comment_start() {
    let config = Config {
        nest_limit: 5,
        flags: Flags {
            ignore_whitespace: true,
            ..Default::default()
        },
    };
    let pattern = "def# starts comment\nghi";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(3),
        char: Cell::new(Some('d')),
        capture_index: Cell::new(0),
        flags: RefCell::new(config.flags),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.peek_space();
} 

#[test]
fn test_peek_space_with_ignore_whitespace_end_of_comment() {
    let config = Config {
        nest_limit: 5,
        flags: Flags {
            ignore_whitespace: true,
            ..Default::default()
        },
    };
    let pattern = "xyz# comment end\nabc";
    let parser = Parser {
        config,
        pattern,
        depth: Cell::new(0),
        pos: Cell::new(20),
        char: Cell::new(Some('a')),
        capture_index: Cell::new(0),
        flags: RefCell::new(config.flags),
        capture_names: RefCell::new(vec![]),
    };
    let _result = parser.peek_space();
}

