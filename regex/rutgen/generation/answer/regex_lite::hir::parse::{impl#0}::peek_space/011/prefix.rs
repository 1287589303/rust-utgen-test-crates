// Answer 0

#[test]
fn test_peek_space_non_whitespace() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            ignore_whitespace: false,
            ..Default::default()
        },
    };
    let pattern = "abc # this is a comment\n";
    let parser = Parser::new(config, pattern);
    parser.peek_space();
}

#[test]
fn test_peek_space_with_space() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            ignore_whitespace: false,
            ..Default::default()
        },
    };
    let pattern = "    def # a comment\n";
    let parser = Parser::new(config, pattern);
    parser.peek_space();
}

#[test]
fn test_peek_space_empty_pattern() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            ignore_whitespace: false,
            ..Default::default()
        },
    };
    let pattern = "";
    let parser = Parser::new(config, pattern);
    parser.peek_space();
}

#[test]
fn test_peek_space_at_nesting_limit() {
    let config = Config {
        nest_limit: 1,
        flags: Flags {
            ignore_whitespace: false,
            ..Default::default()
        },
    };
    let pattern = "(a # comment)\n";
    let parser = Parser::new(config, pattern);
    parser.peek_space();
}

#[test]
fn test_peek_space_multiple_capture_groups() {
    let config = Config {
        nest_limit: 10,
        flags: Flags {
            ignore_whitespace: false,
            ..Default::default()
        },
    };
    let pattern = "(capture1)# comment\n(capture2)";
    let parser = Parser::new(config, pattern);
    parser.peek_space();
}

