// Answer 0

#[test]
fn test_utf8_enabled() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.utf8(true);
}

#[test]
fn test_utf8_disabled() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.utf8(false);
}

