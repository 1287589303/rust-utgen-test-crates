// Answer 0

#[test]
fn test_bump_space_with_ignore_whitespace_and_done() {
    let config = Config {
        flags: Flags {
            ignore_whitespace: true,
            ..Default::default()
        },
        nest_limit: 5,
    };
    let pattern = "   # comment line\n   # another comment\n";
    let parser = Parser::new(config, pattern);
    parser.bump_space();
}

#[test]
fn test_bump_space_with_ignore_whitespace_done_and_only_whitespace() {
    let config = Config {
        flags: Flags {
            ignore_whitespace: true,
            ..Default::default()
        },
        nest_limit: 5,
    };
    let pattern = "   \n\t\t\r\n   ";
    let parser = Parser::new(config, pattern);
    parser.bump_space();
}

#[test]
fn test_bump_space_with_ignore_whitespace_done_and_mixed() {
    let config = Config {
        flags: Flags {
            ignore_whitespace: true,
            ..Default::default()
        },
        nest_limit: 5,
    };
    let pattern = "   # This is a comment\n   # Another comment\n\t \n";
    let parser = Parser::new(config, pattern);
    parser.bump_space();
}

#[test]
fn test_bump_space_with_ignore_whitespace_done_and_edge_case() {
    let config = Config {
        flags: Flags {
            ignore_whitespace: true,
            ..Default::default()
        },
        nest_limit: 5,
    };
    let pattern = " # Single line comment ending\n";
    let parser = Parser::new(config, pattern);
    parser.bump_space();
}

