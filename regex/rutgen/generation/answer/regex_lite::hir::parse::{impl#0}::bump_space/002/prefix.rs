// Answer 0

#[test]
fn test_bump_space_with_whitespace() {
    let config = Config {
        flags: Flags {
            ignore_whitespace: true,
            ..Default::default()
        },
        nest_limit: 10,
    };
    let pattern = "   # comment\n  a";
    let mut parser = Parser::new(config, pattern);
    parser.bump_space();
}

#[test]
fn test_bump_space_with_multiple_whitespace_and_comment() {
    let config = Config {
        flags: Flags {
            ignore_whitespace: true,
            ..Default::default()
        },
        nest_limit: 10,
    };
    let pattern = "   # this is a comment\n   # another comment\nb";
    let mut parser = Parser::new(config, pattern);
    parser.bump_space();
}

#[test]
fn test_bump_space_with_trailing_non_whitespace() {
    let config = Config {
        flags: Flags {
            ignore_whitespace: true,
            ..Default::default()
        },
        nest_limit: 10,
    };
    let pattern = "    # leading whitespace and comment\nc";
    let mut parser = Parser::new(config, pattern);
    parser.bump_space();
}

#[test]
fn test_bump_space_with_empty_line_before() {
    let config = Config {
        flags: Flags {
            ignore_whitespace: true,
            ..Default::default()
        },
        nest_limit: 10,
    };
    let pattern = "# a comment line\n    d";
    let mut parser = Parser::new(config, pattern);
    parser.bump_space();
}

