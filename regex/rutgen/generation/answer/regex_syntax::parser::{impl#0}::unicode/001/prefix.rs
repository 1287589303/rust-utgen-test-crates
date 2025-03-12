// Answer 0

#[test]
fn test_unicode_enable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.unicode(true);
}

#[test]
fn test_unicode_disable() {
    let mut parser_builder = ParserBuilder::new();
    parser_builder.unicode(false);
}

