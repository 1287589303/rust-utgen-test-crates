// Answer 0

#[test]
fn test_peek_space_ignore_whitespace_true_with_comment() {
    let config = Config { nest_limit: 10, flags: Flags { ignore_whitespace: true, ..Default::default() } };
    let pattern = "abc # this is a comment\nxyz";
    let parser = Parser::new(config, pattern);
    let _ = parser.peek_space();
}

#[test]
fn test_peek_space_ignore_whitespace_true_no_comment() {
    let config = Config { nest_limit: 10, flags: Flags { ignore_whitespace: true, ..Default::default() } };
    let pattern = "abc xyz";
    let parser = Parser::new(config, pattern);
    let _ = parser.peek_space();
}

#[test]
fn test_peek_space_ignore_whitespace_true_with_multiple_comments() {
    let config = Config { nest_limit: 10, flags: Flags { ignore_whitespace: true, ..Default::default() } };
    let pattern = "abc # comment 1\n# comment 2\nxyz";
    let parser = Parser::new(config, pattern);
    let _ = parser.peek_space();
}

#[test]
fn test_peek_space_ignore_whitespace_true_with_newline_before_non_whitespace() {
    let config = Config { nest_limit: 10, flags: Flags { ignore_whitespace: true, ..Default::default() } };
    let pattern = "abc \n# comment\n xyz";
    let parser = Parser::new(config, pattern);
    let _ = parser.peek_space();
}

#[test]
fn test_peek_space_ignore_whitespace_true_with_inner_comment() {
    let config = Config { nest_limit: 10, flags: Flags { ignore_whitespace: true, ..Default::default() } };
    let pattern = "abc # comment\n #another comment\nxyz";
    let parser = Parser::new(config, pattern);
    let _ = parser.peek_space();
}

