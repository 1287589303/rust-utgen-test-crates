// Answer 0

#[test]
fn test_ignore_whitespace_enable() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(true);
}

#[test]
fn test_ignore_whitespace_disable() {
    let mut builder = ParserBuilder::new();
    builder.ignore_whitespace(false);
}

