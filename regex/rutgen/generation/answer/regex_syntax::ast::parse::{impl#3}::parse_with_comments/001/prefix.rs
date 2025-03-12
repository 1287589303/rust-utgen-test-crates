// Answer 0

#[test]
fn test_parse_with_comments_basic_characters() {
    let mut parser = Parser::new();
    let pattern = "abc";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_escaped_characters() {
    let mut parser = Parser::new();
    let pattern = "ab\\c";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_character_class() {
    let mut parser = Parser::new();
    let pattern = "[a-z]";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_group() {
    let mut parser = Parser::new();
    let pattern = "(abc)";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_repetition_star() {
    let mut parser = Parser::new();
    let pattern = "a*";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_repetition_plus() {
    let mut parser = Parser::new();
    let pattern = "a+";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_repetition_question() {
    let mut parser = Parser::new();
    let pattern = "a?";
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_octals() {
    let mut parser = Parser::new();
    let pattern = "\\x41"; // A in hex
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_hexadecimals() {
    let mut parser = Parser::new();
    let pattern = "\\u{0041}"; // A in Unicode
    parser.parse_with_comments(pattern);
}

#[test]
fn test_parse_with_comments_with_comments() {
    let mut parser = Parser::new();
    let pattern = "abc# this is a comment\n";
    parser.parse_with_comments(pattern);
}

#[test]
#[should_panic]
fn test_parse_with_comments_too_many_nested_parentheses() {
    let mut parser = Parser::new();
    let pattern = "(((((((((())))))))))"; // Exceeds max nested limit
    parser.parse_with_comments(pattern);
}

