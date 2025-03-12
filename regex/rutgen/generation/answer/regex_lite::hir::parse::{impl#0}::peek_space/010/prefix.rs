// Answer 0

#[test]
fn test_peek_space_with_ignore_whitespace_true() {
    let config = Config { size_limit: None, nest_limit: 10, flags: Flags { ignore_whitespace: true, ..Default::default() } };
    let pattern = "a b c";
    let parser = Parser::new(config, pattern);
    let _ = parser.peek_space();
}

#[test]
fn test_peek_space_with_ignore_whitespace_true_non_whitespace() {
    let config = Config { size_limit: None, nest_limit: 10, flags: Flags { ignore_whitespace: true, ..Default::default() } };
    let pattern = "   a b c"; // Starts with whitespace, but 'a' is non-whitespace
    let parser = Parser::new(config, pattern);
    parser.pos.set(0); // Set position to 0
    let _ = parser.peek_space();
}

#[test]
fn test_peek_space_with_ignore_whitespace_true_non_whitespace_middle() {
    let config = Config { size_limit: None, nest_limit: 10, flags: Flags { ignore_whitespace: true, ..Default::default() } };
    let pattern = "a   b c";
    let parser = Parser::new(config, pattern);
    parser.pos.set(1); // Set position after 'a'
    let _ = parser.peek_space();
}

#[test]
fn test_peek_space_with_ignore_whitespace_true_non_whitespace_end() {
    let config = Config { size_limit: None, nest_limit: 10, flags: Flags { ignore_whitespace: true, ..Default::default() } };
    let pattern = "abc def";
    let parser = Parser::new(config, pattern);
    parser.pos.set(3); // Set position after 'abc'
    let _ = parser.peek_space();
}

