// Answer 0

#[test]
fn test_dot_matches_new_line_enable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.dot_matches_new_line(true);
}

#[test]
fn test_dot_matches_new_line_disable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.dot_matches_new_line(false);
}

